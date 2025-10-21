// # 1. core

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_core {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
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
        #[allow(dead_code)]
        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Type of the queue obtained by adding an element of type `T` to
            /// this queue.
            type PushBack<T>: $q<$($g_lt ,)* $($g ,)*>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the element at the front of the queue.
            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the queue that would be obtained by popping the `Front` element of the queue.
            type Back: $q<$($g_lt ,)* $($g ,)*>;

            /// Number of elements in the queue.
            const LEN: usize;

            /// Pushes the `element` to the back of this queue and returns the resulting queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            fn push<T>(self, x: T) -> Self::PushBack<T>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;


            /// Number of elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
        }

        // # empty

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
        /// ```ignore
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
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)*> $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Creates a new empty queue.
            pub fn new() -> Self {
                Self { phantom: Default::default() }
            }
        }

        impl<$($g_lt ,)* $($g ,)*> Default for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<$($g_lt ,)* $($g ,)*> core::fmt::Debug for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", stringify!($empty))
            }
        }

        impl<$($g_lt ,)* $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* Elem, Self>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = $empty<$($g_lt ,)* $($g ,)*>;

            type Back = Self;

            const LEN: usize = 0;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::from_fb(x, self)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // # pair

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
        /// ```ignore
        /// use orx_meta::queue::*;
        ///
        /// let queue = Queue::new(42);
        /// assert_eq!(queue.len(), 1);
        ///
        /// let queue = Queue::new(42).push(true).push('x').push("foo");
        /// assert_eq!(queue.len(), 4);
        /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
        /// ```
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $pair<$($g_lt ,)* $($g ,)* Front, Back>
        where
            Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            Back: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            f: Front,
            b: Back,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, $empty<$($g_lt ,)* $($g ,)*>>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn new(front: F) -> Self {
                $pair::from_fb(front, $empty::new())
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from_fb(f: F, b: B) -> Self {
                Self {
                    phantom: Default::default(),
                    f,
                    b,
                }
            }

            // ref

            /// Returns a reference to the element in the front of the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
                &self.f
            }

            /// Returns a reference to the queue including elements of this queue
            /// excluding the element in the front.
            ///
            /// Note that accessing elements of the queue is always by `front`, while
            /// `back` allows to access elements in all positions of the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
                &self.b
            }

            // mut

            /// Returns a mutable reference to the element in the front of the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
                &mut self.f
            }

            /// Returns a mutable reference to the queue including elements of this queue
            /// excluding the element in the front.
            ///
            /// Note that mutating elements of the queue is always by `front_mut`, while
            /// `back_mut` allows to access elements in all positions of the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            /// ```compile_fail ignore
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
            /// ```ignore
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
                (&mut self.f, &mut self.b)
            }

            // into

            /// Consumes the queue and returns its front element.
            ///
            /// Equivalent to `queue.pop().0`.
            pub fn into_front(self) -> F {
                self.f
            }

            /// Consumes the queue and returns the queue including elements of this queue
            /// except for the element in the front.
            ///
            /// Equivalent to `queue.pop().1`.
            pub fn into_back(self) -> B {
                self.b
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
            /// ```ignore
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
                (self.f, self.b)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> core::fmt::Debug for $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: core::fmt::Debug,
            B: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?}, {:?})", stringify!($pair), self.f, self.b)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* F, B::PushBack<Elem>>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = B;

            const LEN: usize = 1 + B::LEN;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::from_fb(self.f, self.b.push(x))
            }

            fn len(&self) -> usize {
                1 + self.b.len()
            }
        }
    };
}
