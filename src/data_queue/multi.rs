use crate::{Empty, MetaQueue, data_composer::DataComposer};
use core::marker::PhantomData;

pub struct DataQueueMulti<D, M, T, U>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
    value: (T, U),
}

impl<D, M, T, U> DataQueueMulti<D, M, T, U>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub fn value(self) -> (T, U)
    where
        M: MetaQueue<Back = Empty>,
    {
        self.value
    }
}
