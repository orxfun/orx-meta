use crate::data_queue::{
    empty::EmptyQueue,
    pair::Pair,
    queue::{NonEmptyQueue, Queue},
};

#[derive(Debug)]
pub struct Single<T>(pub(super) T);

impl<T> Queue for Single<T> {
    type Push<X> = Pair<T, Single<X>>;

    type Front = T;

    type Back = EmptyQueue;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, Single(x))
    }

    fn len(&self) -> usize {
        1
    }
}

impl<T> NonEmptyQueue for Single<T> {
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }
}
