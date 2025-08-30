use crate::{
    meta_composition::{MetaComposable, MetaComposition},
    meta_queue::{empty::Empty, queue::MetaQueue},
};

#[derive(Default)]
pub struct MetaQueueComposition;

impl MetaComposition for MetaQueueComposition {
    type Empty = Empty;

    type Compose<X, Y>
        = <X as MetaComposable>::Compose<Y>
    where
        X: MetaComposable;
}

impl<Q: MetaQueue> MetaComposable for Q {
    type Compose<X> = <Self as MetaQueue>::Push<X>;
}
