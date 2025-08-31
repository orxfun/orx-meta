use super::super::{data_composer::DataComposer, empty::Empty, meta_queue::MetaQueue};
use crate::Never;
use core::marker::PhantomData;

pub struct DataQueueMulti<D, M, T, U>
where
    D: DataComposer,
    M: MetaQueue,
{
    p: PhantomData<(D, M)>,
    data: D::Multi<T, U>,
}

impl<D, M, T, U> DataQueueMulti<D, M, T, U>
where
    D: DataComposer,
    M: MetaQueue,
{
    pub(super) fn new(data: D::Multi<T, U>) -> Self {
        Self {
            p: PhantomData,
            data,
        }
    }

    pub fn value(self) -> D::Multi<T, U>
    where
        M: MetaQueue<Front = Never, Back = Empty>,
    {
        self.data
    }

    pub fn add(self, next: M::Front) -> DataQueueMulti<D, M::Back, D::Multi<T, U>, M::Front> {
        let data = D::multi_to_multi(self.data, next);
        DataQueueMulti::new(data)
    }
}
