pub trait StQueue {
    type PushBack<T>: StQueue;

    type Front;

    type Back: StQueue;

    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn front(&self) -> &Self::Front;

    fn front_mut(&mut self) -> &mut Self::Front;

    fn into_front(self) -> Self::Front;

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
