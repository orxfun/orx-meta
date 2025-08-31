use crate::stack::{
    empty::EmptyStack,
    pair::Pair,
    stack::{NonEmptyStack, Stack},
};

#[derive(Debug, Clone, Copy)]
pub struct Single<T>(pub(super) T);

impl<T> Stack for Single<T> {
    type PushBack<X> = Pair<Self, X>;

    type Front = EmptyStack;

    type Back = T;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Pair(self, x)
    }

    fn len(&self) -> usize {
        1
    }
}

impl<T> NonEmptyStack for Single<T> {
    fn back(&self) -> &Self::Back {
        &self.0
    }

    fn pop_back(self) -> (Self::Back, Self::Front) {
        (self.0, EmptyStack)
    }
}
