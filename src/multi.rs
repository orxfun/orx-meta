use crate::{MetaQueue, One};
use core::marker::PhantomData;

/// A meta queue containing at least two types.
pub struct Multi<F, B> {
    p: PhantomData<(F, B)>,
}

impl<F, B> Default for Multi<F, B> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<F, B> MetaQueue for Multi<F, B>
where
    B: MetaQueue,
{
    type Push<X> = Multi<F, B::Push<X>>;

    type Front = One<F>;

    type Back = B;
}
