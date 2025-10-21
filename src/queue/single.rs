use crate::queue::{Queue, StQueue};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SingletonQueue<F>(F);

impl<F> StQueue for SingletonQueue<F> {
    type PushBack<T> = Queue<F, SingletonQueue<T>>;

    type Front = Self;

    type Back = Self;

    const LEN: usize = 0;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        (self.0, SingletonQueue(element)).into()
    }
}

impl<T> SingletonQueue<T> {
    /// Creates a new empty queue.
    pub fn new(element: T) -> Self {
        Self(element)
    }
}
