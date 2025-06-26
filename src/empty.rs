use crate::{MetaQueue, Never, One};

/// An empty [`MetaQueue`].
#[derive(Default)]
pub struct Empty;

impl MetaQueue for Empty {
    type Push<X> = One<X>;

    type Extend<X>
        = X
    where
        X: MetaQueue;

    type Front = Never;

    type Back = Empty;
}
