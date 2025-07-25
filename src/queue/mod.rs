#[cfg(test)]
mod tests;

mod composition;
mod empty;
mod pair;
mod queue;
mod single;

pub use empty::EmptyQueue;
pub use pair::Pair;
pub use queue::{NonEmptyQueue, Queue};
pub use single::Single;
