use crate::queue::{non_empty::Queue, st_queue::StQueue};

/// An empty queue.
///
/// In order to build a queue, we can start with an empty queue
///
/// * `EmptyQueue::new().push(42).push(true).push('x').push("foo")`
///
/// or we can start with non-empty queue when we are sure that there is at least one element:
///
/// * `Queue::push(42).push(true).push('x').push("foo")`
///
/// Both of the above are identical.
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
/// assert_eq!(queue.len(), 1);
///
/// let (num, queue) = queue.pop();
/// assert_eq!(num, 42);
/// assert!(queue.is_empty());
///
/// let queue = EmptyQueue.push(42).push(true);
/// let (num, queue) = queue.pop();
/// let (flag, queue) = queue.pop();
/// assert_eq!(queue, EmptyQueue::new());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct EmptyQueue;

impl StQueue for EmptyQueue {
    type PushBack<T> = Queue<T, EmptyQueue>;

    type Front = Self;

    type Back = Self;

    const LEN: usize = 0;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue::new(element)
    }
}

impl EmptyQueue {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self
    }
}
