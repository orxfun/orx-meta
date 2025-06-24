use crate::MetaQueue;

/// An empty [`MetaQueue`].
#[derive(Default)]
pub struct Empty;

impl MetaQueue for Empty {
    type Push<X> = Self;

    type Front = Self;

    type Back = Self;
}
