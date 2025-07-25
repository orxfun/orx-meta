use crate::{
    Never,
    data_queue::{queue::Queue, single::Single},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Empty;

impl Queue for Empty {
    type Push<X> = Single<X>;

    type Front = Never;

    type Back = Empty;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}
