use crate::{
    Never,
    queue::{queue::Queue, single::Single},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyQueue;

impl Queue for EmptyQueue {
    type Push<X> = Single<X>;

    type Front = Never;

    type Back = EmptyQueue;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}
