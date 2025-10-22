use crate::queue::{QueueSingle, StQueue};
use core::marker::PhantomData;

pub struct QueueBuilder<Target>
where
    Target: StQueue,
{
    target: PhantomData<Target>,
}

impl<Target> Default for QueueBuilder<Target>
where
    Target: StQueue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Target> QueueBuilder<Target>
where
    Target: StQueue,
{
    pub fn new() -> Self {
        Self {
            target: Default::default(),
        }
    }

    pub fn push(
        self,
        element: Target::Front,
    ) -> QueueBuilding<Target, Target::Back, QueueSingle<Target::Front>> {
        QueueBuilding::new(QueueSingle::new(element))
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
    fn new(current: Current) -> Self {
        Self {
            target: Default::default(),
            remaining: Default::default(),
            current,
        }
    }

    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilding<Target, Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilding::new(self.current.push(element))
    }

    pub fn finish(self) -> Current
    where
        Target: StQueue<Front = Current::Front, Back = Current::Back>,
    {
        self.current
    }
}
