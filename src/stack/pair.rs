use crate::stack::stack::{NonEmptyStack, Stack};

#[derive(Debug, Clone, Copy)]
pub struct Pair<F, B>(pub(super) F, pub(super) B);

impl<F, B> Stack for Pair<F, B>
where
    F: Stack,
{
    type PushBack<X> = Pair<Self, X>;

    type Front = F;

    type Back = B;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Pair(self, x)
    }

    fn len(&self) -> usize {
        self.0.len() + 1
    }
}
impl<F, B> NonEmptyStack for Pair<F, B>
where
    F: Stack,
{
    fn back(&self) -> &Self::Back {
        &self.1
    }

    fn pop_back(self) -> (Self::Back, Self::Front) {
        (self.1, self.0)
    }
}
