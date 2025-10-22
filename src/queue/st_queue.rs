/// A strongly typed non-empty queue of heterogeneous elements.
///
/// There exist two implementations:
/// * [`QueueSingle`] which includes exactly one element, and
/// * [`Queue`] containing multiple (>=2) elements.
///
/// Also see [`define_queue`] macro to define a queue of heterogeneous elements
/// all of which exhibit a common behavior, or implement a common set of traits.
/// For more information, please see
/// * examples [3_composition_idea](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)
///   and [5_solution_with_macros](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md); and
/// * the article [zero cost composition](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15).
///
/// [`define_queue`]: crate::define_queue
/// [`QueueSingle`]: crate::queue::QueueSingle
/// [`Queue`]: crate::queue::Queue
#[allow(clippy::len_without_is_empty)]
pub trait StQueue {
    /// Type of the queue obtained by adding an element of type `Elem` to this queue.
    type PushBack<Elem>: StQueue;

    /// Type of the element at the front of the queue.
    type Front;

    /// Type of the queue that would be obtained by popping the `Front` element of the queue.
    type Back: StQueue;

    /// Number of elements in the queue.
    const LEN: usize;

    /// Pushes the `element` and returns the resulting queue.
    ///
    /// *Type of the resulting queue is known by the generic associated type `Self::PushBack<Elem>`.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true).push('x');
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
    ///
    /// let queue = queue.push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>;

    /// Pushes the `element` and returns the resulting queue.
    ///
    /// *This method is provided for convention. Length of the queue is actually known by the constant `Self::LEN`.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
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
    #[inline(always)]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns a reference to the element in the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.front(), &42);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.front(), &42);
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue.front(), &true);
    ///
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    /// assert_eq!(queue.front(), &'x');
    ///
    /// let (c, queue) = queue.pop();
    /// assert_eq!(c, 'x');
    /// assert_eq!(queue.front(), &"foo");
    ///
    /// let s = queue.pop();
    /// assert_eq!(s, "foo");
    /// ```
    fn front(&self) -> &Self::Front;

    /// Returns a mutable reference to the element in the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42).push(true).push('x');
    ///
    /// *queue.front_mut() *= 2;
    /// *queue.back_mut().front_mut() = false;
    /// *queue.back_mut().back_mut().front_mut() = 'y';
    ///
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y'));
    /// ```
    fn front_mut(&mut self) -> &mut Self::Front;

    /// Consumes the queue and returns the element in the front of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_front(), 42);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo".to_string());
    /// assert_eq!(queue.into_front(), 42);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo".to_string());
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    ///
    /// assert_eq!(queue.into_front(), 'x');
    /// ```
    fn into_front(self) -> Self::Front;
}
