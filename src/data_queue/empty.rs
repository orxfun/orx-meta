use crate::{MetaQueue, data_composer::DataComposer, data_queue::one::DataQueueOne};
use core::marker::PhantomData;

pub struct DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
}

impl<D, M> DataQueueEmpty<D, M>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub fn add(self, next: M::Front) -> DataQueueOne<D, M::Back, M::Front> {
        todo!()
    }
}
