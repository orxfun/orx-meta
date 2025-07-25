use crate::meta_queue::{empty::Empty, pair::Pair, queue::MetaQueue};
use core::marker::PhantomData;

/// A meta queue containing exactly one type, which is the generic argument `T`.
pub struct Single<T> {
    p: PhantomData<T>,
}

impl<T> Default for Single<T> {
    fn default() -> Self {
        Self {
            p: Default::default(),
        }
    }
}

impl<T> MetaQueue for Single<T> {
    type Push<X> = Pair<T, Single<X>>;

    type Front = T;

    type Back = Empty;
}
