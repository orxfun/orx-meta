use crate::queue::{EmptyQueue, queue_push::QueuePush};

pub struct Queue<F, B>
where
    B: QueuePush,
{
    front: F,
    back: B,
}

impl<F, B> QueuePush for Queue<F, B>
where
    B: QueuePush,
{
    type PushBack<T> = Queue<F, B::PushBack<T>>;

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
    B: QueuePush,
{
    fn new(front: F, back: B) -> Self {
        Self { front, back }
    }

    // into

    pub fn pop(self) -> (F, B) {
        (self.front, self.back)
    }
}
