pub trait StQueue {
    type PushBack<T>: StQueue;

    type Front;

    type Back: StQueue;

    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
