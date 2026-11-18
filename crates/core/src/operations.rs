//! Traits precede every implementation. Transport admission and deduplication
//! belong to the translator; operations receive only admitted `Heard` content.

use crate::{
    Configuration, Refusal,
    emission::{Delivery_Data, PostInThread_Data, PostRoot_Data, Posting_Data, Rooting_Data, Deliver_Data},
    memory::State,
    operation::{
        Operation, Outcome, Operation_Opening_Data, Operation_Saying_Data, Operation_Hearing_Data,
        Outcome_Opening_Data, Outcome_Saying_Data, Outcome_Hearing_Data, Configuring_Data,
    },
    meta_signal::Setting,
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
    async fn attempt(&mut self, operation: Operation) -> Outcome;
}

/// The domain owner holds its two capabilities as independent data-bearing parts.
pub struct Messenger<Memory, Emission> {
    pub memory: Memory,
    pub emission: Emission,
}
