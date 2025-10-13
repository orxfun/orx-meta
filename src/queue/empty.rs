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

impl EmptyQueue {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self
    }

    // pub fn push<T>(self, x: T);

    /// Number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = EmptyQueue::new();
    /// assert_eq!(queue.len(), 0);
    ///
    /// let queue = queue.push(42);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let queue = queue.push(true).push('x');
    /// assert_eq!(queue.len(), 3);
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue.len(), 2);
    ///
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let (char, queue) = queue.pop();
    /// assert_eq!(char, 'x');
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        0
    }

    /// Returns true if the queue is empty; equivalent to `queue.len() == 0`.
    pub const fn is_empty(&self) -> bool {
        true
    }
}
