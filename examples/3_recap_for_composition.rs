// draw

trait Draw {
    fn draw(&self);
}

// queue trait

trait QueueMeta: Draw {
    type PushBack<T>: QueueMeta
    where
        T: Draw;

    type Front: Draw;

    type Back: QueueMeta;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw;
}

// queue implementations: empty

struct EmptyQueue;

impl QueueMeta for EmptyQueue {
    type PushBack<T>
        = Queue<T, EmptyQueue>
    where
        T: Draw;

    type Front = Self;

    type Back = Self;

    fn len(&self) -> usize {
        0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw,
    {
        Queue::new(element)
    }
}

impl EmptyQueue {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self
    }
}

impl Draw for EmptyQueue {
    fn draw(&self) {}
}

// queue implementations: non-empty

struct Queue<F, B>
where
    F: Draw,
    B: QueueMeta,
{
    front: F,
    back: B,
}

impl<F, B> QueueMeta for Queue<F, B>
where
    F: Draw,
    B: QueueMeta,
{
    type PushBack<T>
        = Queue<F, B::PushBack<T>>
    where
        T: Draw;

    type Front = F;

    type Back = B;

    fn len(&self) -> usize {
        1 + self.back.len()
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw,
    {
        Queue {
            front: self.front,
            back: self.back.push(element),
        }
    }
}

impl<F> Queue<F, EmptyQueue>
where
    F: Draw,
{
    pub fn new(front: F) -> Self {
        Self {
            front,
            back: EmptyQueue,
        }
    }
}

impl<F, B> Draw for Queue<F, B>
where
    F: Draw,
    B: QueueMeta,
{
    fn draw(&self) {
        self.front.draw();
        self.back.draw();
    }
}

fn main() {}
