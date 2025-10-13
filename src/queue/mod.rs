#[cfg(test)]
mod tests;

mod builder;
mod combined;
mod core;
mod plain_queue;
mod queue_of;
mod tuple;

pub use plain_queue::{Empty, Multi, NonEmptyQueue, Queue, QueueBuilder, Single};
