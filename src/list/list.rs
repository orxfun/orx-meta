pub trait List {
    type Push<X>: List;

    type Front;

    type Back: List;

    fn push<X>(self, x: X) -> Self::Push<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait NonEmptyQueue: List {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}
