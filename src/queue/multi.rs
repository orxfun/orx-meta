use super::meta_queue::MetaQueue;
use crate::composition::Composition;
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

    type Extend<X>
        = Multi<F, B::Extend<X>>
    where
        X: MetaQueue;

    type Front = F;

    type Back = B;

    type Map<C: Composition> = C::Multi<F, B::Map<C>>;
}
