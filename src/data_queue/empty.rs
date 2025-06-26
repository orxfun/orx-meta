use crate::{MetaQueue, data_composer::DataComposer};
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
    //
}
