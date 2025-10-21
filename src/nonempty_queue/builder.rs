use crate::nonempty_queue::{QueueSingle, StQueue};
use core::marker::PhantomData;

pub struct QueueBuilder<Target>
where
    Target: StQueue,
{
    target: PhantomData<Target>,
}

impl<Target> QueueBuilder<Target>
where
    Target: StQueue,
{
    pub fn new() -> Self {
        Self {
            target: PhantomData,
        }
    }

    pub fn push(
        self,
        element: Target::Front,
    ) -> QueueBuilding<Target::Back, QueueSingle<Target::Front>> {
        QueueBuilding {
            remaining: PhantomData,
            current: QueueSingle::new(element),
        }
    }
}

pub struct QueueBuilding<Remaining, Current>
where
    Remaining: StQueue,
    Current: StQueue,
{
    remaining: PhantomData<Remaining>,
    current: Current,
}

impl<Remaining, Current> QueueBuilding<Remaining, Current>
where
    Remaining: StQueue,
    Current: StQueue,
{
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilding<Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilding {
            remaining: PhantomData,
            current: self.current.push(element),
        }
    }

    pub fn finish(self) -> Current
    where
        Remaining: StQueue<Back = Remaining>,
    {
        self.current
    }
}
