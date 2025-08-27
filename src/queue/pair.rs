use crate::queue::queue::{NonEmptyQueue, Queue};

#[derive(Debug, Clone, Copy)]
pub struct Pair<F, B>(pub(super) F, pub(super) B);

impl<F, B> Queue for Pair<F, B>
where
    B: Queue,
{
    type PushBack<X> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, self.1.push_back(x))
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

impl<F, B> NonEmptyQueue for Pair<F, B>
where
    B: Queue,
{
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}
