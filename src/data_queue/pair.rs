use crate::data_queue::queue::{NonEmptyQueue, Queue};

#[derive(Debug)]
pub struct Pair<F, B>(pub(super) F, pub(super) B);

impl<F, B> Queue for Pair<F, B>
where
    B: Queue,
{
    type Push<X> = Pair<F, B::Push<X>>;

    type Front = F;

    type Back = B;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, self.1.push(x))
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
