use super::{empty::Empty, meta_queue::MetaQueue, multi::Multi};
use core::marker::PhantomData;

/// A meta queue containing exactly one type, which is the generic argument `T`.
pub struct One<T> {
    p: PhantomData<T>,
}

impl<T> Default for One<T> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<T> MetaQueue for One<T> {
    type Push<X> = Multi<T, One<X>>;

    type Extend<X>
        = Multi<T, X>
    where
        X: MetaQueue;

    type Front = T;

    type Back = Empty;
}
