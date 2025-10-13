#[macro_export]
macro_rules! define_queue_tuple_transformation {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];
    ) => {
        // tuple - 1

        #[allow(dead_code)]
        impl<$($g_lt ,)* X1, $($g ,)*> $single<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> X1 {
                self.f
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> &X1 {
                &self.f
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> &mut X1 {
                &mut self.f
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1> From<X1> for $single<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: X1) -> Self {
                $single::new(x)
            }
        }

        // tuple - 2

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2> $pair<$($g_lt ,)* $($g ,)* X1, $single<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2) {
                (&self.f, &self.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
                (&mut self.f, &mut self.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2> From<(X1, X2)> for $pair<$($g_lt ,)* $($g ,)* X1, $single<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2)) -> Self {
                $pair::new(x.0, $single::new(x.1))
            }
        }

        // tuple - 3

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $single<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.f, self.b.f, self.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
                (&self.f, &self.b.f, &self.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3> From<(X1, X2, X3)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $single<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $single::new(x.2)))
            }
        }

        // tuple - 4

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $single<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $single<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $single::new(x.3))))
            }
        }

        // tuple - 5

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $single<$($g_lt ,)* $($g ,)* X5>>
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $single<$($g_lt ,)* $($g ,)* X5>>
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $single::new(x.4)))))
            }
        }

        // tuple - 6

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $single<$($g_lt ,)* $($g ,)* X6>>
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $single<$($g_lt ,)* $($g ,)* X6>>
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $single::new(x.5))))))
            }
        }

        // tuple - 7

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6, $single<$($g_lt ,)* $($g ,)* X7>>
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6, &mut X7) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6, $single<$($g_lt ,)* $($g ,)* X7>>
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $single::new(x.6)))))))
            }
        }

        // tuple - 8

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7, X8>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6,
                                    $pair<$($g_lt ,)* $($g ,)* X7, $single<$($g_lt ,)* $($g ,)* X8>>
                                >
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Consumes the queue and returns the corresponding tuple.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.into_tuple(), (42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.into_tuple(), (42, true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.into_tuple(), (42, true, 'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.into_tuple(), (42, true, 'x', "foo"));
            /// ```
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let queue = Empty::new().push(42);
            /// assert_eq!(queue.as_tuple(), (&42));
            ///
            /// let queue = Empty::new().push(42).push(true);
            /// assert_eq!(queue.as_tuple(), (&42, &true));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x');
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
            ///
            /// let queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f, &self.b.b.b.b.b.b.b.f)
            }

            /// Returns a representation of the queue as a tuple of mutable references of its elements.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            ///
            /// let mut queue = Empty::new().push(42);
            /// let (a) = queue.as_tuple_mut();
            /// *a += 1;
            /// assert_eq!(queue.as_tuple(), (&43));
            ///
            /// let mut queue = Empty::new().push(42).push(true);
            /// let (a, b) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// assert_eq!(queue.as_tuple(), (&43, &false));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x');
            /// let (a, b, c) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y'));
            ///
            /// let mut queue = Empty::new().push(42).push(true).push('x').push("foo");
            /// let (a, b, c, d) = queue.as_tuple_mut();
            /// *a += 1;
            /// *b = false;
            /// *c = 'y';
            /// *d = "bar";
            /// assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));
            /// ```
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6, &mut X7, &mut X8) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6,
                                    $pair<$($g_lt ,)* $($g ,)* X7, $single<$($g_lt ,)* $($g ,)* X8>>
                                >
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $pair::new(x.6, $single::new(x.7))))))))
            }
        }
    };
}
