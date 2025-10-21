use crate::nonempty_queue::test_queue::{QueueSingle, StQueue};
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
    ) -> QueueBuilding<Target, Target::Back, QueueSingle<Target::Front>> {
        QueueBuilding {
            target: PhantomData,
            remaining: PhantomData,
            current: QueueSingle::new(element),
        }
    }
}

pub struct QueueBuilding<Target, Remaining, Current>
where
    Target: StQueue,
    Remaining: StQueue,
    Current: StQueue,
{
    target: PhantomData<Target>,
    remaining: PhantomData<Remaining>,
    current: Current,
}

impl<Target, Remaining, Current> QueueBuilding<Target, Remaining, Current>
where
    Target: StQueue,
    Remaining: StQueue,
    Current: StQueue,
{
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilding<Target, Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilding {
            target: PhantomData,
            remaining: PhantomData,
            current: self.current.push(element),
        }
    }

    pub fn finish(self) -> Current
    where
        Target: StQueue<Front = Current::Front, Back = Current::Back>,
    {
        self.current
    }
}
