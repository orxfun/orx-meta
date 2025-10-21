pub trait StQueue {
    type PushBack<Elem>: StQueue;

    type Front;

    type Back: StQueue;

    const LEN: usize;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn front(&self) -> &Self::Front;

    fn front_mut(&mut self) -> &mut Self::Front;

    fn into_front(self) -> Self::Front;
}
