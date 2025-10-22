use crate::queue::{Queue, StQueue};

/// A statically-typed queue containing exactly one element of type `Front`.
///
/// See also the other [`StQueue`] implementation [`Queue`] which can be
/// created by pushing a second element to this queue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QueueSingle<Front> {
    pub(super) front: Front,
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
    /// Creates a new statically-typed queue [`StQueue`] containing exactly one `element`.
    ///
    /// Alternatively, we can use multiple element queue's [`new`]. This is for convenience to
    /// allows to work with a single queue type while coding.
    ///
    /// [`new`]: crate::queue::Queue::new
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue: QueueSingle<u32> = QueueSingle::new(42);
    /// assert_eq!(queue.len(), 1);
    /// assert_eq!(queue.front(), &42);
    ///
    /// // alternatively, we can use `Queue::new`:
    /// let queue: QueueSingle<u32> = Queue::new(42);
    /// assert_eq!(queue.len(), 1);
    /// assert_eq!(queue.front(), &42);
    /// ```
    #[inline(always)]
    pub fn new(element: F) -> Self {
        Self { front: element }
    }

    /// Pops and returns the element in the front of this queue.
    ///
    /// Since this element contains only one element, there is no remaining queue once the
    /// front is popped. Therefore, the return type is only the element rather than a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue: QueueSingle<u32> = QueueSingle::new(42);
    ///
    /// let num = queue.pop();
    /// assert_eq!(num, 42);
    /// ```
    #[inline(always)]
    pub fn pop(self) -> F {
        self.front
    }
}
