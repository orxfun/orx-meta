use crate::MetaQueue;
use core::marker::PhantomData;

/// A meta queue containing at least two types.
pub struct Multi<F, U> {
    p: PhantomData<(F, U)>,
}

impl<F, U> Default for Multi<F, U> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<F, U> MetaQueue for Multi<F, U>
where
    U: MetaQueue,
{
    type Push<X> = Self;

    type Front = Self;

    type Back = Self;
}
