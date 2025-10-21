use crate::nonempty_queue::test_queue::{Queue, StQueue};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct QueueSingle<F> {
    pub(super) front: F,
}

impl<F> StQueue for QueueSingle<F> {
    type PushBack<T> = Queue<F, QueueSingle<T>>;

    type Front = Self;

    type Back = Self;

    const LEN: usize = 1;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        (self.front, QueueSingle::new(element)).into()
    }
}

impl<F> QueueSingle<F> {
    /// Creates a new empty queue.
    pub fn new(element: F) -> Self {
        Self { front: element }
    }

    pub fn into_inner(self) -> F {
        self.front
    }
}
