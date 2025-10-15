/// A strongly typed queue of heterogeneous elements.
///
/// There exist two implementations:
/// * [`EmptyQueue`]
/// * [`Queue`]: a non-empty queue where the front element is of type `F`
///   and the remaining elements is also a queue of type `B: StQueue`.
///
/// Also see [`define_queue`] macro to define a queue of heterogeneous elements
/// all of which exhibit a common behavior, or implement a common set of traits.
/// You may see its use cases [here](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md).
///
/// [`define_queue`]: crate::define_queue
/// [`EmptyQueue`]: crate::queue::EmptyQueue
/// [`Queue`]: crate::queue::Queue
pub trait StQueue {
    /// Type of the queue obtained by adding an element of type `T` to
    /// this queue.
    type PushBack<T>: StQueue;

    /// Type of the element at the front of the queue.
    type Front;

    /// Type of the queue that would be obtained by popping the `Front` element of the queue.
    type Back: StQueue;

    /// Number of elements in the queue.
    const LEN: usize;

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
    /// let queue = EmptyQueue::new().push(42);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let queue = Queue::new(42).push(true).push('x');
    /// assert_eq!(queue.len(), 3);
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue.len(), 2);
    ///
    /// let queue = queue.push(true);
    /// assert_eq!(queue.len(), 3);
    /// ```
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns true if `queue.len() == 0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes the `element` to the back of this queue and returns the resulting queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = EmptyQueue::new();
    ///
    /// let queue = queue.push(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = queue.push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = queue.push('x');
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
    ///
    /// let queue = queue.push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
