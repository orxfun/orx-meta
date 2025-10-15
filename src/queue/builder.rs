use crate::queue::{EmptyQueue, st_queue::StQueue};
use core::marker::PhantomData;

pub struct QueueBuilder<Remaining, Current = EmptyQueue>
where
    Remaining: StQueue,
    Current: StQueue,
{
    current: Current,
    remaining: PhantomData<Remaining>,
}

impl<Remaining> QueueBuilder<Remaining, EmptyQueue>
where
    Remaining: StQueue,
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
    Remaining: StQueue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Remaining, Current> QueueBuilder<Remaining, Current>
where
    Remaining: StQueue,
    Current: StQueue,
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
        Remaining: StQueue<Back = Remaining>,
    {
        self.current
    }
}
