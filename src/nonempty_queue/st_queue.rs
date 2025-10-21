pub trait StQueue {
    type PushBack<T>: StQueue;

    type Front;

    type Back: StQueue;

    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns true if `queue.len() == 0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
