use super::queue::MetaQueue;
use core::marker::PhantomData;

/// A meta queue containing at least two types.
pub struct Pair<F, B> {
    p: PhantomData<(F, B)>,
}

impl<F, B> Default for Pair<F, B> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<F, B> MetaQueue for Pair<F, B>
where
    B: MetaQueue,
{
    type Push<X> = Pair<F, B::Push<X>>;

    type Front = F;

    type Back = B;
}
