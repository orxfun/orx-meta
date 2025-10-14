use crate::queue::{meta::QueueMeta, non_empty::Queue};

/// An empty queue.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// let queue = EmptyQueue::new();
/// assert!(queue.is_empty());
///
/// let queue = EmptyQueue.push(42);
/// let (num, queue) = queue.pop();
/// assert_eq!(num, 42);
/// assert!(queue.is_empty());
///
/// let queue = EmptyQueue.push(42).push(true);
/// let (num, queue) = queue.pop();
/// let (flag, queue) = queue.pop();
/// assert!(queue.is_empty());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct EmptyQueue;

impl QueueMeta for EmptyQueue {
    type PushBack<T> = Queue<T, EmptyQueue>;

    type Front = Self;

    type Back = Self;

    fn len(&self) -> usize {
        0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue::single(element)
    }
}

impl EmptyQueue {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self
    }
}
