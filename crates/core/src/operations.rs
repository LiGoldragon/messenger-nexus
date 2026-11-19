//! Traits precede every implementation. Transport admission and deduplication
//! belong to the translator; operations receive only admitted `Heard` content.

use crate::{
    Configuration, Content, Refusal, Subject, Thread,
    emission::{
        Deliver_Data, Delivery_Data, PostInThread_Data, PostRoot_Data, Posting_Data, Rooting_Data,
    },
    memory::State,
    meta_signal::Setting,
    operation::{
        Configuring_Data, Observing_Data, Operation, Operation_Hearing_Data,
        Operation_Opening_Data, Operation_Saying_Data, Outcome, Outcome_Hearing_Data,
        Outcome_Opening_Data, Outcome_Saying_Data,
    },
};

/// The operation's sole route into its persisted domain state.
pub trait Remembering {
    fn recall(&mut self) -> Result<State, Refusal>;
    fn replace(&mut self, state: State) -> Result<(), Refusal>;
}

/// One external call per typed emission, with its observed outcome.
#[allow(async_fn_in_trait)]
pub trait Emitting {
    async fn post_root(&mut self, request: PostRoot_Data) -> Rooting_Data;
    async fn post_in_thread(&mut self, request: PostInThread_Data) -> Posting_Data;
    async fn deliver(&mut self, request: Deliver_Data) -> Delivery_Data;
}

#[allow(async_fn_in_trait)]
pub trait Opening {
    async fn open(&mut self, request: Operation_Opening_Data) -> Outcome_Opening_Data;
}
#[allow(async_fn_in_trait)]
pub trait Saying {
    async fn say(&mut self, request: Operation_Saying_Data) -> Outcome_Saying_Data;
}
#[allow(async_fn_in_trait)]
pub trait Hearing {
    async fn hear(&mut self, request: Operation_Hearing_Data) -> Outcome_Hearing_Data;
}
pub trait Configuring {
    fn configure(&mut self, setting: Setting) -> Configuring_Data;
}
pub trait Observing {
    fn observe(&mut self) -> Result<Configuration, Refusal>;
}

#[allow(async_fn_in_trait)]
pub trait Attempting: Opening + Saying + Hearing + Configuring + Observing {
    async fn attempt(&mut self, operation: Operation) -> Outcome {
        match operation {
            Operation::Opening(request) => Outcome::Opening(self.open(request).await),
            Operation::Saying(request) => Outcome::Saying(self.say(request).await),
            Operation::Hearing(request) => Outcome::Hearing(self.hear(request).await),
            Operation::Configuring(setting) => Outcome::Configuring(self.configure(setting)),
            Operation::Observing => match self.observe() {
                Ok(configuration) => Outcome::Observing(Observing_Data::Observed(configuration)),
                Err(refusal) => Outcome::Observing(Observing_Data::Refused(refusal)),
            },
        }
    }
}

/// The domain owner holds its two capabilities as independent data-bearing parts.
pub struct Messenger<Memory, Emission> {
    pub memory: Memory,
    pub emission: Emission,
}

impl<Memory: Remembering, Emission: Emitting> Opening for Messenger<Memory, Emission> {
    async fn open(&mut self, request: Operation_Opening_Data) -> Outcome_Opening_Data {
        if !matches!(&request.content, Content::Text(_)) {
            return Outcome_Opening_Data::Refused(Refusal::UnsupportedContent);
        }
        let mut state = match self.memory.recall() {
            Ok(state) => state,
            Err(refusal) => return Outcome_Opening_Data::Refused(refusal),
        };
        let configuration = &state.configuration;
        if configuration.homeserver_option.is_none()
            || configuration.room_id_option.is_none()
            || configuration.credential_path_option.is_none()
            || configuration.crypto_store_path_option.is_none()
        {
            return Outcome_Opening_Data::Refused(Refusal::Unconfigured);
        }
        let rooted = self
            .emission
            .post_root(PostRoot_Data {
                subject: request.subject.clone(),
                content: request.content,
            })
            .await;
        match rooted {
            Rooting_Data::Rooted(thread_id) => {
                state.thread_vector.push(Thread {
                    thread_id: thread_id.clone(),
                    address: request.address,
                    subject: request.subject,
                });
                match self.memory.replace(state) {
                    Ok(()) => Outcome_Opening_Data::Opened(thread_id),
                    Err(refusal) => Outcome_Opening_Data::Refused(refusal),
                }
            }
            Rooting_Data::Unrooted(failure) => {
                Outcome_Opening_Data::Refused(Refusal::MatrixFailure(failure))
            }
        }
    }
}

impl<Memory: Remembering, Emission: Emitting> Saying for Messenger<Memory, Emission> {
    async fn say(&mut self, request: Operation_Saying_Data) -> Outcome_Saying_Data {
        if !matches!(&request.content, Content::Text(_)) {
            return Outcome_Saying_Data::Refused(Refusal::UnsupportedContent);
        }
        let state = match self.memory.recall() {
            Ok(state) => state,
            Err(refusal) => return Outcome_Saying_Data::Refused(refusal),
        };
        if !state
            .thread_vector
            .iter()
            .any(|thread| thread.thread_id == request.thread_id)
        {
            return Outcome_Saying_Data::Refused(Refusal::UnknownThread(request.thread_id));
        }
        match self
            .emission
            .post_in_thread(PostInThread_Data {
                thread_id: request.thread_id,
                content: request.content,
            })
            .await
        {
            Posting_Data::Posted => Outcome_Saying_Data::Said,
            Posting_Data::Unposted(failure) => {
                Outcome_Saying_Data::Refused(Refusal::MatrixFailure(failure))
            }
        }
    }
}

impl<Memory: Remembering, Emission: Emitting> Hearing for Messenger<Memory, Emission> {
    async fn hear(&mut self, request: Operation_Hearing_Data) -> Outcome_Hearing_Data {
        let text = match request.content {
            Content::Text(text) => text,
            Content::Image(_) => return Outcome_Hearing_Data::Refused(Refusal::UnsupportedContent),
        };
        let state = match self.memory.recall() {
            Ok(state) => state,
            Err(refusal) => return Outcome_Hearing_Data::Refused(refusal),
        };
        if state.configuration.flow_client_path_option.is_none() {
            return Outcome_Hearing_Data::Refused(Refusal::Unconfigured);
        }
        let destination = match request.thread_id_option {
            Some(thread_id) => match state
                .thread_vector
                .into_iter()
                .find(|thread| thread.thread_id == thread_id)
            {
                Some(thread) => Deliver_Data {
                    address: thread.address,
                    subject: thread.subject,
                    text,
                },
                None => return Outcome_Hearing_Data::Refused(Refusal::UnknownThread(thread_id)),
            },
            None => {
                let configuration = state.configuration;
                match configuration.address_option {
                    Some(address) => match configuration.room_id_option {
                        Some(room_id) => Deliver_Data {
                            address,
                            subject: Subject(room_id.0),
                            text,
                        },
                        None => return Outcome_Hearing_Data::Refused(Refusal::Unconfigured),
                    },
                    None => return Outcome_Hearing_Data::Refused(Refusal::Unconfigured),
                }
            }
        };
        match self.emission.deliver(destination).await {
            Delivery_Data::Delivered => Outcome_Hearing_Data::Delivered,
            Delivery_Data::Undelivered(failure) => {
                Outcome_Hearing_Data::Refused(Refusal::FlowFailure(failure))
            }
        }
    }
}

impl<Memory: Remembering, Emission> Configuring for Messenger<Memory, Emission> {
    fn configure(&mut self, setting: Setting) -> Configuring_Data {
        let mut state = match self.memory.recall() {
            Ok(state) => state,
            Err(refusal) => return Configuring_Data::Refused(refusal),
        };
        let configuration = &mut state.configuration;
        match setting {
            Setting::Homeserver(value) => configuration.homeserver_option = Some(value),
            Setting::RoomId(value) => configuration.room_id_option = Some(value),
            Setting::LivingUserId(value) => configuration.living_user_id_option = Some(value),
            Setting::CredentialPath(value) => configuration.credential_path_option = Some(value),
            Setting::CryptoStorePath(value) => configuration.crypto_store_path_option = Some(value),
            Setting::Communicator(value) => configuration.address_option = Some(value),
            Setting::FlowClientPath(value) => configuration.flow_client_path_option = Some(value),
        }
        match self.memory.replace(state) {
            Ok(()) => Configuring_Data::Configured,
            Err(refusal) => Configuring_Data::Refused(refusal),
        }
    }
}

impl<Memory: Remembering, Emission> Observing for Messenger<Memory, Emission> {
    fn observe(&mut self) -> Result<Configuration, Refusal> {
        self.memory.recall().map(|state| state.configuration)
    }
}

impl<Memory: Remembering, Emission: Emitting> Attempting for Messenger<Memory, Emission> {}
