//! The Messenger domain's typed vocabulary and operation contracts.
//!
//! Runtime adapters supply Memory and Emission; this library speaks no text
//! unless its consumer explicitly enables the `datom` feature.

extern crate self as messenger;

#[path = "generated/messenger.rs"]
mod vocabulary;
pub use vocabulary::*;
#[path = "generated/signal.rs"]
pub mod signal;
#[path = "generated/meta-signal.rs"]
pub mod meta_signal;
#[path = "generated/memory.rs"]
pub mod memory;
#[path = "generated/operation.rs"]
pub mod operation;
#[path = "generated/emission.rs"]
pub mod emission;

pub mod operations;
