use crate::product::{empty::Empty, pop::Pop, push::Push};
use core::marker::PhantomData;

pub struct Builder<Remaining, Current = Empty>
where
    Remaining: Push,
    Current: Push,
{
    current: Current,
    remaining: PhantomData<Remaining>,
}

impl<Remaining> Builder<Remaining, Empty>
where
    Remaining: Push,
{
    pub fn new() -> Self {
        Self {
            current: Empty,
            remaining: Default::default(),
        }
    }
}

impl<Remaining> Default for Builder<Remaining, Empty>
where
    Remaining: Push,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Remaining, Current> Builder<Remaining, Current>
where
    Remaining: Push,
    Current: Push,
{
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> Builder<Remaining::PopBack, Current::PushBack<Remaining::Front>>
    where
        Remaining: Pop,
    {
        Builder {
            current: self.current.push_back(element),
            remaining: Default::default(),
        }
    }

    pub fn finish(self) -> Current
    where
        Remaining: Pop<Back = Remaining>,
    {
        self.current
    }
}
