use crate::{Empty, MetaQueue, data_composer::DataComposer, data_queue::one::DataQueueOne};
use core::marker::PhantomData;

pub struct DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
    data: D::Empty,
}

impl<D, M> DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub fn value(self) -> D::Empty
    where
        M: MetaQueue<Back = Empty>,
    {
        D::empty()
    }

    pub fn add(self, next: M::Front) -> DataQueueOne<D, M::Back, M::Front> {
        DataQueueOne::new(D::one(next))
    }
}
