#![allow(dead_code)]
use crate::queue::{QueueSingle, StQueue};

/// A queue containing multiple (>= 2) elements.
///
/// It is composed of two parts:
/// * `f: Front` is the element in the front of the queue;
/// * `b: Back` is the queue of remaining elements except for the one in the front.
///   It can be:
///   * either a [`QueueSingle`] containing exactly one element in which case length of this
///     queue is 2,
///   * or a [`Queue`] containing multiple elements, in which case length of this queue is
///     greater than 2, `1 + self.b.len()`.
///
/// Note that `Queue::new(element)` gives a `QueueSingle` with one element. In order to create
/// a queue of multiple elements, we need to push at least one more element, such as
/// `Queue::new(elem1).push(elem2)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Queue<Front, Back>
where
    Back: StQueue,
{
    f: Front,
    b: Back,
}

impl<F, B> StQueue for Queue<F, B>
where
    B: StQueue,
{
    type PushBack<Elem> = Queue<F, B::PushBack<Elem>>;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    #[inline(always)]
    fn front(&self) -> &F {
        &self.f
    }

    #[inline(always)]
    fn front_mut(&mut self) -> &mut F {
        &mut self.f
    }

    #[inline(always)]
    fn into_front(self) -> F {
        self.f
    }

    #[inline(always)]
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem> {
        Queue::from_fb(self.f, self.b.push(element))
    }
}

impl<F> Queue<F, QueueSingle<F>> {
    /// Creates a [`QueueSingle`] with exactly one `element`.
    ///
    /// Note that `Queue::new` is equivalent to `QueueSingle::new`. It is introduced for
    /// convenience allowing us to work only with the multiple element queue type `Queue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    /// use orx_meta::queue_of;
    ///
    /// // creates a QueueSingle
    /// let queue: QueueSingle<u32> = Queue::new(42);
    /// assert_eq!(queue.len(), 1);
    /// assert_eq!(queue.front(), &42);
    ///
    /// // creates a Queue when we push at least one more element
    /// let queue: Queue<u32, QueueSingle<char>> = Queue::new(42).push('x');
    /// assert_eq!(queue.len(), 2);
    ///
    /// let queue: Queue<u32, Queue<char, Queue<bool, QueueSingle<String>>>>
    ///   = Queue::new(42).push('x').push(true).push("foo".to_string());
    /// assert_eq!(queue.as_tuple(), (&42, &'x', &true, &"foo".to_string()));
    ///
    /// // equivalently, we can use the queue_of macro to create the type
    /// let queue: queue_of!(u32, char, bool, String)
    ///   = Queue::new(42).push('x').push(true).push("foo".to_string());
    /// assert_eq!(queue.as_tuple(), (&42, &'x', &true, &"foo".to_string()));
    /// ```
    #[inline(always)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(element: F) -> QueueSingle<F> {
        QueueSingle::new(element)
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    #[inline(always)]
    pub(super) fn from_fb(front: F, back: B) -> Self {
        Self { f: front, b: back }
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    // ref

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
    /// // assert_eq!(queue.back(), ??); // wont' compile, QueueSingle has no back
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
    /// // assert_eq!(queue.back(), ??);  // wont' compile, QueueSingle has no back
    ///
    /// let s = queue.pop();
    /// assert_eq!(s, "foo");
    /// ```
    #[inline(always)]
    pub fn back(&self) -> &B {
        &self.b
    }

    // mut

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
    #[inline(always)]
    pub fn back_mut(&mut self) -> &mut B {
        &mut self.b
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
    #[inline(always)]
    pub fn front_back_mut(&mut self) -> (&mut F, &mut B) {
        (&mut self.f, &mut self.b)
    }

    // into

    /// Consumes the queue and returns the queue including elements of this queue
    /// except for the element in the front.
    ///
    /// Equivalent to `queue.pop().1`.
    #[inline(always)]
    pub fn into_back(self) -> B {
        self.b
    }

    /// Consumes the queue and returns the tuple of its front and back:
    ///
    /// * **front** is the element in the front of this queue.
    /// * **back** is the queue including all elements of this queue except
    ///   for the front element. In other words, it is the queue obtained by
    ///   popping the front element.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_meta::queue::*;
    ///
    /// let queue = Queue::new(42);
    /// let num = queue.pop(); // QueueSingle::pop just returns the front
    /// assert_eq!(num, 42);
    ///
    /// let queue = Queue::new(42).push(true).push('x').push("foo");
    ///
    /// let (num, queue) = queue.pop(); // Queue::pop returns (front, back)
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
    /// let s = queue.pop();
    /// assert_eq!(s, "foo");
    /// ```
    #[inline(always)]
    pub fn pop(self) -> (F, B) {
        (self.f, self.b)
    }
}

// tuple

type S<F> = QueueSingle<F>;

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
        (self.f, self.b.front)
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
        (&self.f, &self.b.front)
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
        (&mut self.f, &mut self.b.front)
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
        (self.f, self.b.f, self.b.b.front)
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
        (&self.f, &self.b.f, &self.b.b.front)
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
        (&mut self.f, &mut self.b.f, &mut self.b.b.front)
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
        (self.f, self.b.f, self.b.b.f, self.b.b.b.front)
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
        (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.front)
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
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.front,
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
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.front,
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
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.front,
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
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.front,
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
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.front,
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
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.front,
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
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.front,
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
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.f,
            self.b.b.b.b.b.b.front,
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
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.front,
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
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.front,
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
            self.f,
            self.b.f,
            self.b.b.f,
            self.b.b.b.f,
            self.b.b.b.b.f,
            self.b.b.b.b.b.f,
            self.b.b.b.b.b.b.f,
            self.b.b.b.b.b.b.b.front,
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
            &self.f,
            &self.b.f,
            &self.b.b.f,
            &self.b.b.b.f,
            &self.b.b.b.b.f,
            &self.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.f,
            &self.b.b.b.b.b.b.b.front,
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
            &mut self.f,
            &mut self.b.f,
            &mut self.b.b.f,
            &mut self.b.b.b.f,
            &mut self.b.b.b.b.f,
            &mut self.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.f,
            &mut self.b.b.b.b.b.b.b.front,
        )
    }
}
