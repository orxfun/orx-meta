use crate::{Empty, MetaQueue, data_composer::DataComposer};
use core::marker::PhantomData;

pub struct DataQueueOne<D, M, T>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
    value: T,
}

impl<D, M, T> DataQueueOne<D, M, T>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub fn value(self) -> T
    where
        M: MetaQueue<Back = Empty>,
    {
        self.value
    }
}
