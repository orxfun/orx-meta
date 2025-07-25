use crate::Never;
use crate::queue::{meta_queue::MetaQueue, single::Single};

/// An empty [`MetaQueue`].
#[derive(Default)]
pub struct Empty;

impl MetaQueue for Empty {
    type Push<X> = Single<X>;

    type Extend<X>
        = X
    where
        X: MetaQueue;

    type Front = Never;

    type Back = Empty;
}
