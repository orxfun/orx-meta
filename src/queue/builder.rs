use crate::queue::{EmptyQueue, meta::QueueMeta};
use core::marker::PhantomData;

pub struct QueueBuilder<Remaining, Current = EmptyQueue>
where
    Remaining: QueueMeta,
    Current: QueueMeta,
{
    current: Current,
    remaining: PhantomData<Remaining>,
}

impl<Remaining> QueueBuilder<Remaining, EmptyQueue>
where
    Remaining: QueueMeta,
{
    pub fn new() -> Self {
        Self {
            current: EmptyQueue,
            remaining: Default::default(),
        }
    }
}

impl<Remaining> Default for QueueBuilder<Remaining, EmptyQueue>
where
    Remaining: QueueMeta,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Remaining, Current> QueueBuilder<Remaining, Current>
where
    Remaining: QueueMeta,
    Current: QueueMeta,
{
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilder<Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilder {
            current: self.current.push(element),
            remaining: Default::default(),
        }
    }

    pub fn finish(self) -> Current
    where
        Remaining: QueueMeta<Back = Remaining>,
    {
        self.current
    }
}
