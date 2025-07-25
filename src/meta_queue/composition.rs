use crate::{
    composition::Composition,
    meta_queue::{empty::Empty, pair::Pair, single::Single},
};

pub struct MetaQueueComposition;

impl Composition for MetaQueueComposition {
    type Empty = Empty;

    type Single<X> = Single<X>;

    type Pair<X, Y> = Pair<X, Y>;

    type Trio<X, Y, Z> = Pair<X, Pair<Y, Z>>;

    fn empty() -> Self::Empty {
        Default::default()
    }

    fn single<X>(_: X) -> Self::Single<X> {
        Default::default()
    }

    fn pair<X, Y>(_: X, _: Y) -> Self::Pair<X, Y> {
        Default::default()
    }

    fn compose_with_single<X, Y>(_: Self::Single<X>, _: Y) -> Self::Pair<X, Y> {
        Default::default()
    }

    fn compose_with_pair<X, Y, Z>(_: Self::Pair<X, Y>, _: Z) -> Self::Trio<X, Y, Z> {
        Default::default()
    }
}
