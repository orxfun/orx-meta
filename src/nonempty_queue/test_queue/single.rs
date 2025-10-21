use crate::nonempty_queue::test_queue::{Queue, StQueue};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct QueueSingle<F> {
    pub(super) front: F,
}

impl<F> StQueue for QueueSingle<F> {
    type PushBack<Elem> = Queue<F, QueueSingle<Elem>>;

    type Front = F;

    type Back = Self;

    const LEN: usize = 1;

    #[inline(always)]
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem> {
        Queue::from_fb(self.front, QueueSingle::new(element))
    }

    #[inline(always)]
    fn front(&self) -> &Self::Front {
        &self.front
    }

    #[inline(always)]
    fn front_mut(&mut self) -> &mut Self::Front {
        &mut self.front
    }

    #[inline(always)]
    fn into_front(self) -> Self::Front {
        self.front
    }
}

impl<F> QueueSingle<F> {
    /// Creates a new empty queue.
    #[inline(always)]
    pub fn new(element: F) -> Self {
        Self { front: element }
    }
}
