use crate::{
    data_composer::DataComposer,
    data_queue::{DataQueueEmpty, DataQueueMulti, DataQueueOne},
};

/// A data composer where the composition of data pieces `a` and `b`
/// is the tuple `(a, b)`.
#[derive(Default)]
pub struct TupleComposer;

impl DataComposer for TupleComposer {
    type Empty = ();

    type One<X> = X;

    type Multi<X, Y> = (X, Y);

    #[inline(always)]
    fn empty() -> Self::Empty {}

    #[inline(always)]
    fn one<X>(x: X) -> Self::One<X> {
        x
    }

    #[inline(always)]
    fn one_to_multi<X, Y>(x: Self::One<X>, y: Y) -> Self::Multi<X, Y> {
        (x, y)
    }

    #[inline(always)]
    fn multi_to_multi<X, Y, Z>(
        (x, y): Self::Multi<X, Y>,
        z: Z,
    ) -> Self::Multi<Self::Multi<X, Y>, Z> {
        ((x, y), z)
    }
}

pub type TupleQueue<M> = DataQueueEmpty<TupleComposer, M>;

pub type TupleQueueOne<M, T> = DataQueueOne<TupleComposer, M, T>;

pub type TupleQueueMulti<M, T, U> = DataQueueMulti<TupleComposer, M, T, U>;

#[cfg(test)]
mod testament {
    use crate::*;
}
