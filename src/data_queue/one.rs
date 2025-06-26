use crate::{
    Empty, MetaQueue, Never, data_composer::DataComposer, data_queue::multi::DataQueueMulti,
};
use core::marker::PhantomData;

pub struct DataQueueOne<D, M, T>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
    data: D::One<T>,
}

impl<D, M, T> DataQueueOne<D, M, T>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub(super) fn new(data: D::One<T>) -> Self {
        Self {
            p: PhantomData,
            data,
        }
    }

    pub fn value(self) -> D::One<T>
    where
        M: MetaQueue<Front = Never, Back = Empty>,
    {
        self.data
    }

    pub fn add(self, next: M::Front) -> DataQueueMulti<D, M::Back, T, M::Front> {
        let data = D::one_to_multi(self.data, next);
        DataQueueMulti::new(data)
    }
}
