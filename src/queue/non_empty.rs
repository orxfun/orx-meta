use crate::queue::{EmptyQueue, st_queue::StQueue};

/// A queue containing at least one element.
///
/// It is composed of two parts represented by its two generic parameters:
///
/// * `front: F`: This is the element in the front of the queue.
/// * `back: B`: This is the queue that would be obtained if the front element
///   is popped. Equivalently, it is the queue of all elements in this queue
///   except for the front element. Note that it can be an [`EmptyQueue`], in
///   which case length of this queue would be one.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// let queue = Queue::new(42);
/// assert_eq!(queue.len(), 1);
///
/// let queue = Queue::new(42).push(true).push('x').push("foo");
/// assert_eq!(queue.len(), 4);
/// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Queue<F, B>
where
    B: StQueue,
{
    front: F,
    back: B,
}

impl<F, B> StQueue for Queue<F, B>
where
    B: StQueue,
{
    type PushBack<T> = Queue<F, B::PushBack<T>>;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue::from((self.front, self.back.push(element)))
    }
}

impl<F> Queue<F, EmptyQueue> {
    /// Creates a new non-empty queue with a single element being the `front`.
    /// Note that the *back* of the queue will be an [`EmptyQueue`].
    ///
    /// In order to build queues with multiple elements, you may use chained [`push`]
    /// calls following `new`.
    ///
    /// [`push`]: StQueue::push
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.len(), 1);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.len(), 4);
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn new(front: F) -> Self {
        Self {
            front,
            back: EmptyQueue,
        }
    }
}

impl<F, B> From<(F, B)> for Queue<F, B>
where
    B: StQueue,
{
    fn from((front, back): (F, B)) -> Self {
        Self { front, back }
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    // ref

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
    /// let (s, queue) = queue.pop();
    /// assert_eq!(s, "foo");
    ///
    /// // does not compile, EmptyQueue::front does not exist ;)
    /// // assert_eq!(queue.front(), ??);
    /// ```
    pub fn front(&self) -> &F {
        &self.front
    }

    /// Returns a reference to the queue including elements of this queue
    /// excluding the element in the front.
    ///
    /// Note that accessing elements of the queue is always by `front`, while
    /// `back` allows to access elements in all positions of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.front(), &42);
    /// assert_eq!(queue.back(), &EmptyQueue);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.back(), &Queue::new(true).push('x').push("foo"));
    /// assert_eq!(queue.front(), &42);
    /// assert_eq!(queue.back().front(), &true);
    /// assert_eq!(queue.back().back().front(), &'x');
    /// assert_eq!(queue.back().back().back().front(), &"foo");
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue.front(), &true);
    /// assert_eq!(queue.back(), &Queue::new('x').push("foo"));
    ///
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    /// assert_eq!(queue.front(), &'x');
    /// assert_eq!(queue.back(), &Queue::new("foo"));
    ///
    /// let (c, queue) = queue.pop();
    /// assert_eq!(c, 'x');
    /// assert_eq!(queue.front(), &"foo");
    /// assert_eq!(queue.back(), &EmptyQueue);
    ///
    /// let (s, queue) = queue.pop();
    /// assert_eq!(s, "foo");
    ///
    /// // does not compile, front & back do not exist for EmptyQueue ;)
    /// // assert_eq!(queue.front(), ??);
    /// // assert_eq!(queue.back(), ??);
    /// ```
    pub fn back(&self) -> &B {
        &self.back
    }

    // mut

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
    pub fn front_mut(&mut self) -> &mut F {
        &mut self.front
    }

    /// Returns a mutable reference to the queue including elements of this queue
    /// excluding the element in the front.
    ///
    /// Note that mutating elements of the queue is always by `front_mut`, while
    /// `back_mut` allows to access elements in all positions of the queue.
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
    pub fn back_mut(&mut self) -> &mut B {
        &mut self.back
    }

    /// Returns a pair of mutable references:
    /// * first to the element in the front of the queue, and
    /// * second to the back queue containing elements except for the front.
    ///
    /// # Safety
    ///
    /// Assume we have a queue of three elements and we want to mutate the first and
    /// third elements as follows.
    ///
    /// However, the following code would not compile.
    ///
    /// ```compile_fail
    /// use orx_meta::queue::*;
    ///
    /// let mut q = Queue::new(3).push(true).push('x');
    ///
    /// let first = q.front_mut();
    /// let third = q.back_mut().back_mut().front_mut();
    ///
    /// // these calls can be made concurrently
    /// *first *= 2;
    /// *third = 'y';
    /// ```
    ///
    /// It is perfectly safe to mutate the first and third elements at the same time.
    /// Actually, we can mutate all of the elements concurrently.
    ///
    /// However, we need to help the compiler to figure this out, which is why we get
    /// the mutable references to the front and back at the same time. With this, the
    /// compiler understands that there is no overlap between them.
    ///
    /// # Examples
    ///
    /// So the following code would compile and work expectedly.
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut q = Queue::new(3).push(true).push('x');
    ///
    /// let (first, q23) = q.front_back_mut();
    /// let third = q23.back_mut().front_mut();
    ///
    /// // these calls can be made concurrently
    /// *first *= 2;
    /// *third = 'y';
    ///
    /// assert_eq!(q.as_tuple(), (&6, &true, &'y'));
    /// ```
    pub fn front_back_mut(&mut self) -> (&mut F, &mut B) {
        (&mut self.front, &mut self.back)
    }

    // into

    /// Consumes the queue and returns its front element.
    ///
    /// Equivalent to `queue.pop().0`.
    pub fn into_front(self) -> F {
        self.front
    }

    /// Consumes the queue and returns the queue including elements of this queue
    /// except for the element in the front.
    ///
    /// Equivalent to `queue.pop().1`.
    pub fn into_back(self) -> B {
        self.back
    }

    /// Consumes the queue and returns the tuple of its front and back:
    ///
    /// * **front** is the element in the front of this queue.
    /// * **back** is the queue including all elements of this queue except
    ///   for the front element. In other words, it is the queue obtained by
    ///   popping the front element. Note that back might be an empty queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue, EmptyQueue::new());
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.front(), &42);
    ///
    /// let (num, queue) = queue.pop();
    /// assert_eq!(num, 42);
    /// assert_eq!(queue, Queue::new(true).push('x').push("foo"));
    ///
    /// let (flag, queue) = queue.pop();
    /// assert_eq!(flag, true);
    /// assert_eq!(queue, Queue::new('x').push("foo"));
    ///
    /// let (c, queue) = queue.pop();
    /// assert_eq!(c, 'x');
    /// assert_eq!(queue, Queue::new("foo"));
    ///
    /// let (s, queue) = queue.pop();
    /// assert_eq!(s, "foo");
    /// assert_eq!(queue, EmptyQueue::new());
    ///
    /// // does not compile, EmptyQueue::pop does not exist ;)
    /// // let (?, queue) = queue.pop();
    /// ```
    pub fn pop(self) -> (F, B) {
        (self.front, self.back)
    }
}

// tuple

type S<F> = Queue<F, EmptyQueue>;

impl<X1> S<X1> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> X1 {
        self.front
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> &X1 {
        &self.front
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> &mut X1 {
        &mut self.front
    }
}

impl<X1, X2> Queue<X1, S<X2>> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2) {
        (self.front, self.back.front)
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2) {
        (&self.front, &self.back.front)
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
        (&mut self.front, &mut self.back.front)
    }
}

impl<X1, X2, X3> Queue<X1, Queue<X2, S<X3>>> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.front, self.back.front, self.back.back.front)
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
        (&self.front, &self.back.front, &self.back.back.front)
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4> Queue<X1, Queue<X2, Queue<X3, S<X4>>>> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5> Queue<X1, Queue<X2, Queue<X3, Queue<X4, S<X5>>>>> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6> Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, S<X6>>>>>> {
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7>
    Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, Queue<X6, S<X7>>>>>>>
{
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
            self.back.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
    ) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7, X8>
    Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, Queue<X6, Queue<X7, S<X8>>>>>>>>
{
    /// Converts the queue into its flat tuple representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.into_tuple(), 42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.into_tuple(), (42, true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
    /// ```
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
            self.back.back.back.back.back.back.front,
            self.back.back.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// assert_eq!(queue.as_tuple(), &42);
    ///
    /// let queue = Queue::new(42).push(true);
    /// assert_eq!(queue.as_tuple(), (&42, &true));
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
    /// ```
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.back.front,
        )
    }
    /// Returns a flat tuple representation of mutable references to elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let mut queue = Queue::new(42);
    /// let a = queue.as_tuple_mut();
    /// *a *= 2;
    /// assert_eq!(queue.as_tuple(), &84);
    ///
    /// let mut queue = Queue::new(42).push(true);
    /// let (a, b) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// assert_eq!(queue.as_tuple(), (&84, &false));
    ///
    /// let mut queue = Queue::new(42).push(true).push('x').push("foo");
    /// let (a, b, c, d) = queue.as_tuple_mut();
    /// *a *= 2;
    /// *b = false;
    /// *c = 'y';
    /// *d = "bar";
    /// assert_eq!(queue.as_tuple(), (&84, &false, &'y', &"bar"));
    /// ```
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
        &mut X8,
    ) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.back.front,
        )
    }
}
