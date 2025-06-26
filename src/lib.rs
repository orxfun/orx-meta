#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests;

mod data_composer;
mod data_queue;
mod empty;
mod meta_queue;
mod multi;
mod never;
mod one;
mod tuple_composer;

pub use empty::Empty;
pub use meta_queue::MetaQueue;
pub use multi::Multi;
pub use never::Never;
pub use one::One;
pub use tuple_composer::{TupleQueue, TupleQueueMulti, TupleQueueOne};
