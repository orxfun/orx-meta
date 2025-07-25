use crate::{Composition, queue::EmptyQueue};

#[derive(Default)]
pub struct QueueComposition;

impl Composition for QueueComposition {
    type Empty = EmptyQueue;

    fn empty() -> Self::Empty {
        EmptyQueue
    }
}
