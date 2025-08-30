pub trait Stack {
    type PushBack<X>: NonEmptyStack;

    type Front: Stack;

    type Back;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait NonEmptyStack: Stack {
    fn back(&self) -> &Self::Back;

    fn pop_back(self) -> (Self::Back, Self::Front);
}
