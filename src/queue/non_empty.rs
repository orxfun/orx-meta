use crate::queue::{EmptyQueue, meta::QueueMeta};

pub struct Queue<F, B>
where
    B: QueueMeta,
{
    front: F,
    back: B,
}

impl<F, B> QueueMeta for Queue<F, B>
where
    B: QueueMeta,
{
    type PushBack<T> = Queue<F, B::PushBack<T>>;

    type Front = F;

    type Back = B;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue::new(self.front, self.back.push(element))
    }
}

impl<F> Queue<F, EmptyQueue> {
    pub(super) fn single(front: F) -> Self {
        Self {
            front,
            back: EmptyQueue,
        }
    }
}

impl<F, B> Queue<F, B>
where
    B: QueueMeta,
{
    fn new(front: F, back: B) -> Self {
        Self { front, back }
    }

    // ref

    pub fn front(&self) -> &F {
        &self.front
    }

    pub fn back(&self) -> &B {
        &self.back
    }

    // mut

    pub fn front_mut(&mut self) -> &mut F {
        &mut self.front
    }

    pub fn back_mut(&mut self) -> &mut B {
        &mut self.back
    }

    // into

    pub fn into_front(self) -> F {
        self.front
    }

    pub fn into_back(self) -> B {
        self.back
    }

    pub fn pop(self) -> (F, B) {
        (self.front, self.back)
    }
}
