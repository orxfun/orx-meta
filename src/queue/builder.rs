#[macro_export]
macro_rules! define_queue_builder {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        /// A type-safe builder for queues such that:
        ///
        /// * `push` can only be called correct number of times with the correct types,
        /// * `finish` can only be called after all elements are pushed.
        ///
        /// It can also be used as a generic builder for any tuple type.
        ///
        /// # Example
        ///
        /// In the following example, we want to build a queue of four elements of types `u32`, `bool`, `char` and `&str` respectively.
        ///
        /// For this, we can create a builder with `QueueBuilder::<MyQueue>::new()` where `MyQueue` is the target type to instantiate.
        ///
        /// ```
        /// use orx_meta::queue::*;
        ///
        /// type MyQueue = Multi<u32, Multi<bool, Multi<char, Single<&'static str>>>>;
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .push("foo")
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
        /// ```
        ///
        /// This provides a convenient way to build complex types without errors and with compiler support.
        /// However, it is not easy to hand-write the type alias for the complex recursive queue type.
        /// Therefore, this builder pattern is most useful when used together with the [`queue_of`] macro.
        /// The above example could be re-written as follows with the `queue_of` macro.
        ///
        /// ```
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, &'static str);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .push("foo")
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
        /// ```
        ///
        /// ## Examples - Type Safety
        ///
        /// Note that this builder pattern is type safe in the sense that neither of the following wrong implementations compiles.
        ///
        /// Here the elements are pushed in the wrong order:
        ///
        /// ```compile_fail
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, &'static str);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(true) // wrong order!
        ///     .push(42)
        ///     .push('x')
        ///     .push("foo")
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
        /// ```
        ///
        /// And here, not all elements are pushed:
        ///
        /// ```compile_fail
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, &'static str);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .finish(); // forgot to push &str
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
        /// ```
        pub struct $builder<$($g_lt ,)* $($g ,)* Remaining, Current>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            cur: Current,
            rem: core::marker::PhantomData<Remaining>,
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)* Remaining> $builder<$($g_lt ,)* $($g ,)* Remaining, $empty<$($g_lt ,)* $($g ,)*>>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Creates a new builder for a target queue.
            ///
            /// The type parameter used when constructing the builder defines the target
            /// type to be constructed.
            ///
            /// The builder makes sure that `finish` can be called only after all elements
            /// required by the target type are pushed.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            /// use orx_meta::queue_of;
            ///
            /// let builder = QueueBuilder::<Empty>::new();
            /// let instance = builder.finish();
            /// assert!(instance.is_empty());
            ///
            /// let builder = QueueBuilder::<Single<u32>>::new(); // or
            /// let builder = QueueBuilder::<queue_of!(u32)>::new();
            /// let instance = builder.push(42).finish();
            /// assert_eq!(instance.as_tuple(), (&42));
            ///
            /// let builder = QueueBuilder::<Multi<u32, Single<bool>>>::new(); // or
            /// let builder = QueueBuilder::<queue_of!(u32, bool)>::new();
            /// let instance = builder.push(42).push(true).finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true));
            ///
            /// let builder = QueueBuilder::<Multi<u32, Multi<bool, Multi<char, Single<&'static str>>>>>::new(); // or
            /// let builder = QueueBuilder::<queue_of!(u32, bool, char, &'static str)>::new();
            /// let instance = builder.push(42).push(true).push('x').push("foo").finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn new() -> Self {
                Self {
                    cur: $empty::new(),
                    rem: Default::default(),
                    phantom: Default::default(),
                }
            }
        }

        impl<$($g_lt ,)* $($g ,)* Remaining> Default for $builder<$($g_lt ,)* $($g ,)* Remaining, $empty<$($g_lt ,)* $($g ,)*>>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<$($g_lt ,)* $($g ,)* Remaining, Current> $builder<$($g_lt ,)* $($g ,)* Remaining, Current>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Pushes the next element to the builder, returns the builder for the remaining elements
            /// to reach the target type.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            /// use orx_meta::queue_of;
            ///
            /// type MyQueue = queue_of!(u32, bool, char, &'static str);
            ///
            /// // remaining:[u32, bool, char, &'static str]
            /// // current: []
            /// let builder = QueueBuilder::<MyQueue>::new();
            ///
            /// // remaining:[bool, char, &'static str]
            /// // current: [u32]
            /// let builder = builder.push(42);
            ///
            /// // remaining:[char, &'static str]
            /// // current: [u32, bool]
            /// let builder = builder.push(true);
            ///
            /// // remaining:[&'static str]
            /// // current: [u32, bool, char]
            /// let builder = builder.push('x');
            ///
            /// // remaining:[] -> we can now call finish
            /// // current: [u32, bool, char, &'static str]
            /// let builder = builder.push("foo");
            ///
            /// let instance = builder.finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
            ///
            /// // it is often more convenient to chain the push calls
            /// let instance = QueueBuilder::<MyQueue>::new()
            ///     .push(42)
            ///     .push(true)
            ///     .push('x')
            ///     .push("foo")
            ///     .finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn push(self, x: Remaining::Front) -> $builder<$($g_lt ,)* $($g ,)* Remaining::Back, Current::PushBack<Remaining::Front>> {
                $builder {
                    cur: self.cur.push(x),
                    rem: Default::default(),
                    phantom: Default::default(),
                }
            }

            /// Consumes the builder and returns the built instance of the target type.
            ///
            /// Note that `finish` can only be called after pushing all required elements of the queue.
            ///
            /// # Examples
            ///
            /// ```
            /// use orx_meta::queue::*;
            /// use orx_meta::queue_of;
            ///
            /// type MyQueue = queue_of!(u32, bool, char, &'static str);
            ///
            /// // remaining:[u32, bool, char, &'static str]
            /// // current: []
            /// let builder = QueueBuilder::<MyQueue>::new();
            ///
            /// // remaining:[bool, char, &'static str]
            /// // current: [u32]
            /// let builder = builder.push(42);
            ///
            /// // remaining:[char, &'static str]
            /// // current: [u32, bool]
            /// let builder = builder.push(true);
            ///
            /// // remaining:[&'static str]
            /// // current: [u32, bool, char]
            /// let builder = builder.push('x');
            ///
            /// // remaining:[] -> we can now call finish
            /// // current: [u32, bool, char, &'static str]
            /// let builder = builder.push("foo");
            ///
            /// let instance = builder.finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
            ///
            /// // it is often more convenient to chain the push calls
            /// let instance = QueueBuilder::<MyQueue>::new()
            ///     .push(42)
            ///     .push(true)
            ///     .push('x')
            ///     .push("foo")
            ///     .finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo"));
            /// ```
            pub fn finish(self) -> Current
            where
                Remaining: $q<$($g_lt ,)* $($g ,)* Back = Remaining>,
            {
                self.cur
            }
        }
    };
}
