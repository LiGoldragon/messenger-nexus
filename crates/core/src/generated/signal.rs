#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Open_Data {
    pub address: flow::Address,
    pub subject: messenger::Subject,
    pub content: messenger::Content,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Say_Data {
    pub thread_id: messenger::ThreadId,
    pub content: messenger::Content,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Heard_Data {
    pub thread_id_option: std::option::Option<messenger::ThreadId>,
    pub content: messenger::Content,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Open(Open_Data),
    Say(Say_Data),
    Heard(Heard_Data),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Opened(messenger::ThreadId),
    Said,
    HeardAccepted,
    Refused(messenger::Refusal),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn open(&self, open: &Open_Data) -> std::result::Result<messenger::ThreadId, Output>;
    fn say(&self, say: &Say_Data) -> std::result::Result<(), Output>;
    fn heard(&self, heard: &Heard_Data) -> std::result::Result<(), Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Output;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Open(carried) => {
                nexus.open(carried).map_or_else(|refusal| refusal, Output::Opened)
            }
            Input::Say(carried) => {
                nexus.say(carried).map_or_else(|refusal| refusal, |()| Output::Said)
            }
            Input::Heard(carried) => {
                nexus
                    .heard(carried)
                    .map_or_else(|refusal| refusal, |()| Output::HeardAccepted)
            }
        }
    }
}
