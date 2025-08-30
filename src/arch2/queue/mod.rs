#[cfg(test)]
mod tests;

mod composable;
mod composition;
mod impl_meta_queue;

pub use composition::QueueComposition;

use crate::{Never, impl_meta_queue};

impl_meta_queue!(
    [],
    [],
    Never,
    Queue,
    NonEmptyQueue,
    EmptyQueue,
    Single,
    Pair
);
