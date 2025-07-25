pub trait Queue {
    type Push<X>: Queue;

    type Front;

    type Back: Queue;

    fn push<X>(self, x: X) -> Self::Push<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait NonEmptyQueue: Queue {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}
