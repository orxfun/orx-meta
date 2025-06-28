use super::super::{
    data_composer::DataComposer, data_queue::one::DataQueueOne, empty::Empty, meta_queue::MetaQueue,
};
use crate::Never;
use core::marker::PhantomData;

pub struct DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
}

impl<D, M> Default for DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D, M> DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub fn new() -> Self {
        Self { p: PhantomData }
    }

    pub fn value(self) -> D::Empty
    where
        M: MetaQueue<Front = Never, Back = Empty>,
    {
        D::empty()
    }

    pub fn add(self, next: M::Front) -> DataQueueOne<D, M::Back, M::Front> {
        DataQueueOne::new(D::one(next))
    }
}
