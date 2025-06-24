use crate::MetaQueue;
use core::marker::PhantomData;

/// A meta queue containing at least two types.
pub struct Multi<T, U> {
    p: PhantomData<(T, U)>,
}

impl<T, U> Default for Multi<T, U> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<T, U> MetaQueue for Multi<T, U> {
    type Push<X> = Self;

    type Front = Self;

    type Back = Self;
}
