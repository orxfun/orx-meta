#[macro_export]
macro_rules! define_queue_core {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];
    ) =>
    {
        /// A strongly typed queue of arbitrary elements.
        ///
        /// There exist three queue implementations:
        /// * `Empty`: empty queue
        /// * `Single<F>`: a queue with a single element of type `F`
        /// * `Multi<F, B>`: a queue with multiple (>1) elements where the front element is of type `F`
        ///   and the remaining elements is a queue of type `B`.
        #[allow(dead_code)]
        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Type of the queue obtained by pushing an element of type `Elem` to the back of the queue.
            type PushBack<Elem>: $q_ne<$($g_lt ,)* $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the element at the front of the queue.
            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the queue that would be obtained by popping the `Front` element of the queue.
            type Back: $q<$($g_lt ,)* $($g ,)*>;

            type Raised: $q<$($g_lt ,)* $($g ,)*>;

            /// Pushes the element `x` to the back of the queue.
            ///
            /// Resulting queue implements [`NonEmptyQueue`].
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new();
            ///
            /// let queue = queue.push(42);
            /// assert_eq!(queue.len(), 1);
            ///
            /// let queue = queue.push(true).push('x');
            /// assert_eq!(queue.len(), 3);
            ///
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            /// ```
            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Number of elements in the queue.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new();
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
            fn len(&self) -> usize;

            /// Returns true if the queue is empty; equivalent to `queue.len() == 0`.
            fn is_empty(&self) -> bool {
                self.len() == 0
            }

            fn raise(self) -> Self::Raised;

            fn from_raised(raised: Self::Raised) -> Self;
        }

        // trait non-empty queue

        /// A strongly typed [`Queue`] that is guaranteed to contain at least one element.
        ///
        /// Among the three queue implementations, [`Single`] and [`Multi`] implements non-empty queue,
        /// while [`Empty`] does not.
        #[allow(dead_code)]
        pub trait $q_ne<$($g_lt ,)* $($g ,)*>: $q<$($g_lt ,)* $($g ,)*>
        where
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
                /// Consumes the queue and returns the front element.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// let queue = Empty::new().push(42);
                /// assert_eq!(queue.into_front(), 42);
                ///
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// assert_eq!(queue.into_front(), 42);
                /// ```
                fn into_front(self) -> Self::Front;

                /// Consumes the queue and returns the queue containing elements except for the front element.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let queue = Empty::new().push(42);
                /// assert_eq!(queue.into_back(), Empty::new());
                ///
                /// // front: 42; back: [true]
                /// let queue = Empty::new().push(42).push(true);
                /// assert_eq!(queue.into_back(), Single::new(true));
                ///
                /// // front: 42; back: [true, 'x']
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// assert_eq!(queue.into_back(), Single::new(true).push('x'));
                /// ```
                fn into_back(self) -> Self::Back;

                /// Consumes the queue and splits it into two pieces:
                /// * the element at the front of the queue, and
                /// * the queue containing elements except for the front element.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let queue = Empty::new().push(42);
                /// let (front, back) = queue.pop();
                /// assert_eq!(front, 42);
                /// assert_eq!(back, Empty::new());
                ///
                /// // front: 42; back: [true]
                /// let queue = Empty::new().push(42).push(true);
                /// let (front, back) = queue.pop();
                /// assert_eq!(front, 42);
                /// assert_eq!(back, Single::new(true));
                ///
                /// // front: 42; back: [true, 'x']
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// let (front, back) = queue.pop();
                /// assert_eq!(front, 42);
                /// assert_eq!(back, Single::new(true).push('x'));
                /// ```
                fn pop(self) -> (Self::Front, Self::Back);

                /// Returns a reference to the element at the front of the queue.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let queue = Empty::new().push(42);
                /// assert_eq!(queue.front(), &42);
                ///
                /// // front: 42; back: [true]
                /// let queue = Empty::new().push(42).push(true);
                /// assert_eq!(queue.front(), &42);
                ///
                /// // front: 42; back: [true, 'x']
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// assert_eq!(queue.front(), &42);
                /// ```
                fn front(&self) -> &Self::Front;

                /// Returns a reference to the queue containing elements of this queue except for the
                /// element at the front.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let queue = Empty::new().push(42);
                /// assert_eq!(queue.back(), &Empty::new());
                ///
                /// // front: 42; back: [true]
                /// let queue = Empty::new().push(42).push(true);
                /// assert_eq!(queue.back(), &Single::new(true));
                ///
                /// // front: 42; back: [true, 'x']
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// assert_eq!(queue.back(), &Single::new(true).push('x'));
                /// ```
                fn back(&self) -> &Self::Back;

                /// Returns a tuple of references to the front and back of the queue:
                /// * front: element at the front of the queue,
                /// * back: queue containing all elements except for the front element.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let queue = Empty::new().push(42);
                /// let (front, back) = queue.front_back();
                /// assert_eq!(front, &42);
                /// assert_eq!(back, &Empty::new());
                ///
                /// // front: 42; back: [true]
                /// let queue = Empty::new().push(42).push(true);
                /// let (front, back) = queue.front_back();
                /// assert_eq!(front, &42);
                /// assert_eq!(back, &Single::new(true));
                ///
                /// // front: 42; back: [true, 'x']
                /// let queue = Empty::new().push(42).push(true).push('x');
                /// let (front, back) = queue.front_back();
                /// assert_eq!(front, &42);
                /// assert_eq!(back, &Single::new(true).push('x'));
                /// ```
                fn front_back(&self) -> (&Self::Front, &Self::Back);

                /// Returns mutable a reference to the element at the front of the queue.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: []
                /// let mut queue = Empty::new().push(42);
                /// *queue.front_mut() += 1;
                /// assert_eq!(queue.front(), &43);
                ///
                /// // front: 42; back: [true]
                /// let mut queue = Empty::new().push(42).push(true);
                /// *queue.front_mut() += 1;
                /// assert_eq!(queue.front(), &43);
                ///
                /// // front: 42; back: [true, 'x']
                /// let mut queue = Empty::new().push(42).push(true).push('x');
                /// *queue.front_mut() += 1;
                /// assert_eq!(queue.front(), &43);
                /// ```
                fn front_mut(&mut self) -> &mut Self::Front;

                /// Returns a mutable reference to the queue containing elements of this queue except for the
                /// element at the front.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: [true]
                /// let mut queue = Empty::new().push(42).push(true);
                /// *queue.back_mut().front_mut() = false;
                /// assert_eq!(queue.as_tuple(), (&42, &false));
                ///
                /// // front: 42; back: [true, 'x']
                /// let mut queue = Empty::new().push(42).push(true).push('x');
                /// *queue.back_mut().front_mut() = false;
                /// *queue.back_mut().back_mut().front_mut() = 'y';
                /// assert_eq!(queue.as_tuple(), (&42, &false, &'y'));
                /// ```
                fn back_mut(&mut self) -> &mut Self::Back;

                /// Returns a tuple of mutable references to the front and back of the queue:
                /// * front: element at the front of the queue,
                /// * back: queue containing all elements except for the front element.
                ///
                /// # Examples
                ///
                /// ```
                /// use orx_meta::queue::*;
                ///
                /// // front: 42; back: [true, 'x']
                /// let mut queue = Empty::new().push(42).push(true).push('x');
                /// let (front, back) = queue.front_back_mut();
                /// *front += 1;
                ///
                /// // recursively destruct the back
                /// let (front, back) = back.front_back_mut();
                /// *front = false;
                ///
                /// // recursively destruct the back
                /// let (front, _back_empty) = back.front_back_mut();
                /// *front = 'y';
                ///
                /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
                /// ```
                fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back);
        }

        // struct empty

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
            type PushBack<Elem> = $single<$($g_lt ,)* $($g ,)* Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = $empty<$($g_lt ,)* $($g ,)*>;

            type Back = Self;

            type Raised = Self;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $single::new(x)
            }

            fn raise(self) -> Self::Raised {
                Default::default()
            }

            fn from_raised(raised: Self::Raised) -> Self{
                raised
            }

            fn len(&self) -> usize {
                0
            }
        }

        // struct single

        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $single<$($g_lt ,)* $($g ,)* Front>
        where
            Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            empty: $empty<$($g_lt ,)* $($g ,)*>,
            f: Front,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $single<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn new(f: F) -> Self {
                Self {
                    phantom: Default::default(),
                    empty: $empty::new(),
                    f,
                }
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> core::fmt::Debug for $single<$($g_lt ,)* $($g ,)* F>
        where
            F: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?})", stringify!($single), self.f)
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* F, $single<$($g_lt ,)* $($g ,)* Elem>>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = $empty<$($g_lt ,)* $($g ,)*>;

            type Raised = $single<$($g_lt ,)* $($g ,)* Self>;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::new(self.f, $single::new(x))
            }

            fn raise(self) -> Self::Raised {
                $single::new(self)
            }

            fn from_raised(raised: Self::Raised) -> Self{
                raised.f
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q_ne<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn into_front(self) -> Self::Front {
                self.f
            }

            fn into_back(self) -> Self::Back {
                self.empty
            }

            fn pop(self) -> (Self::Front, Self::Back) {
                (self.f, $empty{ phantom: Default::default() })
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn back(&self) -> &Self::Back {
                &self.empty
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.empty)
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.empty
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.empty)
            }
        }

        // struct pair

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

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn new(f: F, b: B) -> Self {
                Self {
                    phantom: Default::default(),
                    f,
                    b,
                }
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

            type Raised = $pair<$($g_lt ,)* $($g ,)* $single<$($g_lt ,)* $($g ,)* F>, B::Raised>;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::new(self.f, self.b.push(x))
            }

            fn raise(self) -> Self::Raised {
                $pair::new($single::new(self.f), self.b.raise())
            }


            fn from_raised(raised: Self::Raised) -> Self{
                let f = raised.f.f;
                let b = B::from_raised(raised.b);
                $pair::new(f, b)
            }

            fn len(&self) -> usize {
                1 + self.b.len()
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q_ne<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn into_front(self) -> Self::Front {
                self.f
            }

            fn into_back(self) -> Self::Back {
                self.b
            }

            fn pop(self) -> (Self::Front, Self::Back) {
                (self.f, self.b)
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn back(&self) -> &Self::Back {
                &self.b
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.b)
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.b
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.b)
            }
        }
    };
}
