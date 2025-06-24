use crate::{MetaQueue, One};

/// An empty [`MetaQueue`].
#[derive(Default)]
pub struct Empty;

impl MetaQueue for Empty {
    type Push<X> = One<X>;

    type Front = Empty;

    type Back = Empty;
}
