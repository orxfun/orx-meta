use super::{
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
    fn multi_to_multi<X, Y, Z>(xy: Self::Multi<X, Y>, z: Z) -> Self::Multi<Self::Multi<X, Y>, Z> {
        (xy, z)
    }
}

/// A type-safe tuple composer such that:
///
/// * the target sequence of types to be stored as recursive tuples is defined by the meta queue `M`,
/// * we can keep calling `add` on the builder until the target sequence is obtained,
/// * we can call `value` iff instances of all types are added to obtain the resulting value.
///
/// # Examples
///
/// TODO
pub type TupleQueue<M> = DataQueueEmpty<TupleComposer, M>;

/// A [`TupleQueue`] currently containing one piece of data, of type `T`.
pub type TupleQueueOne<M, T> = DataQueueOne<TupleComposer, M, T>;

/// A [`TupleQueue`] currently containing multiple pieces of data of types `T` and `U`,
pub type TupleQueueMulti<M, T, U> = DataQueueMulti<TupleComposer, M, T, U>;
