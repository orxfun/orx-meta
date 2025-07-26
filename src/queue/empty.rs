use crate::{
    Never,
    queue::{queue::Queue, single::Single},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyQueue;

impl Queue for EmptyQueue {
    type PushBack<X> = Single<X>;

    type Front = Never;

    type Back = EmptyQueue;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}
