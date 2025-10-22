/// [orx_meta](https://crates.io/crates/orx-meta) crate defines statically-typed queues with heterogeneous elements.
/// See [`StQueue`], [`QueueSingle`] and [`Queue`].
/// Further, it provides the [`QueueBuilder`] as a generic builder for complex types and [`queue_of`] for conveniently
/// aliasing complex queue types.
///
/// These are useful for several use cases; however, not sufficient for zero-cost abstraction.
/// For more information, please see
/// * examples [3_composition_idea](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)
///   and [5_solution_with_macros](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md); and
/// * the article [zero cost composition](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15).
///
/// The `define_macro` allows us to re-create all above-mentioned queue types and code with the following additions:
/// * Queue types might have custom generic parameters, and/or lifetime parameters.
/// * But most importantly, we can limit elements that can be included in the queue with a custom trait or traits.
///   In other words, we can add trait bounds to elements that can be pushed to the queue. This allows us to achieve
///   zero-cost composition.
///
/// # Example - Recreating and Renaming
///
/// The following is the simplest usage of this macro.
///
/// ```
/// orx_meta::define_queue!(
///     queue => [ MyQueue ; MySingleQueue, MyMultiQueue ];
/// );
///
/// let queue = MyMultiQueue::new(42).push(true).push('x');
/// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
/// ```
/// It just recreates [`StQueue`] trait with name `MyQueue`, [`QueueSingle`] implementation as `MySingleQueue` and
/// [`Queue`] implementation as `MyMultiQueue`.
///
/// Everything will be the same other than renaming.
///
/// Notice that defining the queue builder and `queue_of` macro are optional. In the following, we also define these
/// again with new names `MyQueueBuilder` and `q_of`.
///
/// ```
/// orx_meta::define_queue!(
///     queue => [ MyQueue ; MySingleQueue, MyMultiQueue ];
///     queue_of => q_of;
///     builder => MyQueueBuilder;
/// );
///
/// let queue = MyQueueBuilder::<q_of!(u32, bool, char)>::new().push(42).push(true).push('x').finish();
/// assert_eq!(queue.as_tuple(), (&42, &true, &'x'));
/// ```
///
/// # Example - Trait Bounds
///
/// The main purpose of this macro; however, is to add trait bounds to the elements that can be contained by the queue.
///
/// Assume for instance, we want all our elements to implement the `Draw` trait.
///
/// We can define this by providing the `elements => [Draw]` block to the macro. Note that we can define a comma-separated
/// list of traits within the brackets.
///
/// In addition to elements of the queue, the design requires the queue types to implement the `Draw` trait as well.
/// This is the central idea of zero-composition.
///
/// Since there are two implementations of the queue, we need to implement `Draw` for these two types:
/// * Single-element queue: This is a trivial implementation. Since the queue contains only one element which implements
///   `Draw`, it exhibits the behavior of that element. This is the **identity**.
/// * Multi-element queue: Here, the queue contains at least two elements. All elements implement `Draw`. We need to define
///   what the draw behavior must be whenever there are multiple elements. This is the **composition**.
///
/// The following provides the entire example.
///
/// ```
/// // draw
///
/// pub trait Draw {
///     fn draw(&self);
/// }
///
/// // example Draw components
///
/// #[derive(Debug)]
/// pub struct Button {
///     pub width: u32,
///     pub height: u32,
///     pub label: String,
/// }
///
/// impl Draw for Button {
///     fn draw(&self) {
///         println!("{self:?}");
///     }
/// }
///
/// #[derive(Debug)]
/// #[allow(dead_code)]
/// struct SelectBox {
///     pub width: u32,
///     pub height: u32,
///     pub options: Vec<String>,
/// }
///
/// impl Draw for SelectBox {
///     fn draw(&self) {
///         println!("{self:?}");
///     }
/// }
///
/// // queue definition
///
/// orx_meta::define_queue!(
///     elements => [ Draw ];
///     queue => [ StScreen ; ScreenSingle, Screen ];
/// );
///
/// impl<F: Draw> Draw for ScreenSingle<F> {
///     fn draw(&self) {
///         self.f.draw();
///     }
/// }
///
/// impl<F, B> Draw for Screen<F, B>
/// where
///     F: Draw,
///     B: StScreen,
/// {
///     fn draw(&self) {
///         self.f.draw();
///         self.b.draw();
///     }
/// }
///
/// let screen = Screen::new(Button {
///     width: 3,
///     height: 4,
///     label: String::from("login"),
/// })
/// .push(Button {
///     width: 4,
///     height: 5,
///     label: String::from("logout"),
/// })
/// .push(SelectBox {
///     width: 10,
///     height: 6,
///     options: vec![String::from("This"), String::from("that")],
/// });
/// screen.draw();
///
/// // prints out:
/// //
/// // Button { width: 3, height: 4, label: "login" }
/// // Button { width: 4, height: 5, label: "logout" }
/// // SelectBox { width: 10, height: 6, options: ["This", "that"] }
/// ```
///
///
///
///
///
///
/// [`StQueue`]: crate::queue::StQueue
/// [`QueueSingle`]: crate::queue::QueueSingle
/// [`Queue`]: crate::queue::Queue
/// [`QueueBuilder`]: crate::queue::QueueBuilder
/// [`queue_of`]: crate::queue_of
#[macro_export]
macro_rules! define_queue {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];

        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );

        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );

        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // # queue_of

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [ ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // # builder

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // # queue_of + builder

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_of!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };
}

// # 1. core

#[doc(hidden)]
#[macro_export]
macro_rules! define_queue_core {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
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
        #[allow(dead_code)]
        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Type of the queue obtained by adding an element of type `Elem` to this queue.
            type PushBack<T>: $q<$($g_lt ,)* $($g ,)*>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the element at the front of the queue.
            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Type of the queue that would be obtained by popping the `Front` element of the queue.
            type Back: $q<$($g_lt ,)* $($g ,)*>;

            /// Number of elements in the queue.
            const LEN: usize;

            /// Pushes the `element` and returns the resulting queue.
            ///
            /// *Type of the resulting queue is known by the generic associated type `Self::PushBack<Elem>`.*
            ///
            /// # Examples
            ///
            /// ```ignore
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
            fn push<T>(self, x: T) -> Self::PushBack<T>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            /// Pushes the `element` and returns the resulting queue.
            ///
            /// *This method is provided for convention. Length of the queue is actually known by the constant `Self::LEN`.*
            ///
            /// # Examples
            ///
            /// ```ignore
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
            /// let s = queue.pop();
            /// assert_eq!(s, "foo");
            /// ```
            fn front(&self) -> &Self::Front;

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
            fn front_mut(&mut self) -> &mut Self::Front;

            /// Consumes the queue and returns the element in the front of the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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

        // # single


        /// A statically-typed queue containing exactly one element of type `Front`.
        ///
        /// See also the other [`StQueue`] implementation [`Queue`] which can be
        /// created by pushing a second element to this queue.
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $empty<$($g_lt ,)* $($g ,)* Front>
        where
            Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            f: Front,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Creates a new statically-typed queue [`StQueue`] containing exactly one `element`.
            ///
            /// Alternatively, we can use multiple element queue's [`new`]. This is for convenience to
            /// allows to work with a single queue type while coding.
            ///
            /// [`new`]: crate::queue::Queue::new
            ///
            /// # Examples
            ///
            /// ```ignore
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
            pub fn new(element: F) -> Self {
                Self {
                    phantom: Default::default(),
                    f: element,
                }
            }

            /// Pops and returns the element in the front of this queue.
            ///
            /// Since this element contains only one element, there is no remaining queue once the
            /// front is popped. Therefore, the return type is only the element rather than a tuple.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use orx_meta::queue::*;
            ///
            /// let queue: QueueSingle<u32> = QueueSingle::new(42);
            ///
            /// let num = queue.pop();
            /// assert_eq!(num, 42);
            /// ```
            #[inline(always)]
            pub fn pop(self) -> F {
                self.f
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> core::fmt::Debug for $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?})", stringify!($empty), self.f)
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* F, $empty<$($g_lt ,)* $($g ,)* Elem>>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = Self;

            const LEN: usize = 1;

            #[inline(always)]
            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::from_fb(self.f, $empty::new(x))
            }

            #[inline(always)]
            fn front(&self) -> &Self::Front {
                &self.f
            }

            #[inline(always)]
            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            #[inline(always)]
            fn into_front(self) -> Self::Front {
                self.f
            }
        }

        // # pair

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

        impl<$($g_lt ,)* F, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, $empty<$($g_lt ,)* $($g ,)* F>>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Creates a [`QueueSingle`] with exactly one `element`.
            ///
            /// Note that `Queue::new` is equivalent to `QueueSingle::new`. It is introduced for
            /// convenience allowing us to work only with the multiple element queue type `Queue`.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            pub fn new(element: F) -> $empty<$($g_lt ,)* $($g ,)* F> {
                $empty::new(element)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from_fb(f: F, b: B) -> Self {
                Self {
                    phantom: Default::default(),
                    f,
                    b,
                }
            }

            // ref

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
            ///   popping the front element.
            ///
            /// # Examples
            ///
            /// ```ignore
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

            #[inline(always)]
            fn front(&self) -> &Self::Front {
                &self.f
            }

            #[inline(always)]
            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            #[inline(always)]
            fn into_front(self) -> Self::Front {
                self.f
            }
        }
    };
}

// # 2. builder

#[doc(hidden)]
#[macro_export]
macro_rules! define_queue_builder {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {

        // builder

        /// A type-safe builder for queues such that:
        ///
        /// * `push` can only be called correct number of times with correct types,
        /// * `finish` can only be called when all elements to reach the `Target` type are pushed.
        ///
        /// Further, since queues can represent any struct, `QueueBuilder` can be used as a generic builder for any struct or tuple.
        ///
        /// # Example
        ///
        /// In the following example, we want to build a queue of four elements of types `u32`, `bool`, `char` and `String` respectively.
        ///
        /// For this, we can create a builder with `QueueBuilder::<MyQueue>::new()` where `MyQueue` is the target type to instantiate.
        ///
        /// ```ignore
        /// use orx_meta::queue::*;
        ///
        /// type MyQueue = Queue<u32, Queue<bool, Queue<char, QueueSingle<String>>>>;
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .push("foo".to_string())
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        ///
        /// This provides a convenient way to build complex types without errors while getting compiler support on what to push next.
        /// However, it is not easy to hand-write the type alias for complex recursive queue types.
        /// Therefore, this builder pattern is most useful when used together with the [`queue_of`] macro.
        /// The above example could be re-written as follows with the `queue_of` macro.
        ///
        /// [`queue_of`]: crate::queue_of
        ///
        /// ```ignore
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, String);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .push("foo".to_string())
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        ///
        /// ## Examples - Type Safety
        ///
        /// Note that this builder pattern is type safe in the sense that neither of the following wrong implementations compiles.
        ///
        /// Here the elements are pushed in the wrong order:
        ///
        /// ```compile_fail ignore
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, String);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(true) // wrong order!
        ///     .push(42)
        ///     .push('x')
        ///     .push("foo".to_string())
        ///     .finish();
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        ///
        /// And here, not all elements are pushed:
        ///
        /// ```compile_fail ignore
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(u32, bool, char, String);
        ///
        /// let instance = QueueBuilder::<MyQueue>::new()
        ///     .push(42)
        ///     .push(true)
        ///     .push('x')
        ///     .finish(); // forgot to push the String
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        pub struct $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            target: core::marker::PhantomData<Target>,
        }

        impl<$($g_lt ,)* $($g ,)* Target> Default for $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<$($g_lt ,)* $($g ,)* Target> $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Creates a new empty builder for the `Target` type defined as the generic argument.
            pub fn new() -> Self {
                Self {
                    target: Default::default(),
                }
            }

            /// Pushes the next `element` to build the target type.
            #[inline(always)]
            pub fn push(
                self,
                element: Target::Front,
            ) -> QueueBuilding<Target, Target::Back, $empty<$($g_lt ,)* $($g ,)* Target::Front>> {
                QueueBuilding::new($empty::new(element))
            }
        }

        // building

        pub struct QueueBuilding<$($g_lt ,)* $($g ,)* Target, Remaining, Current>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            target: core::marker::PhantomData<Target>,
            remaining: core::marker::PhantomData<Remaining>,
            current: Current,
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)* Target, Remaining, Current> QueueBuilding<$($g_lt ,)* $($g ,)* Target, Remaining, Current>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn new(current: Current) -> Self {
                Self {
                    target: Default::default(),
                    remaining: Default::default(),
                    current: current,
                    phantom: Default::default(),
                }
            }

            /// Pushes the next `element` to build the target type.
            #[inline(always)]
            pub fn push(
                self,
                element: Remaining::Front,
            ) -> QueueBuilding<Target, Remaining::Back, Current::PushBack<Remaining::Front>> {
                QueueBuilding::new(self.current.push(element))
            }

            /// Completes the builder and returns the built target type.
            #[inline(always)]
            pub fn finish(self) -> Current
            where
                Target: $q<$($g_lt ,)* $($g ,)* Front = Current::Front, Back = Current::Back>,
            {
                self.current
            }
        }
    };
}

// # 3. tuple

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_tuple_transformation {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        // tuple - 1

        #[allow(dead_code)]
        impl<$($g_lt ,)* X1, $($g ,)*> $empty<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> X1 {
                self.f
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> &X1 {
                &self.f
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> &mut X1 {
                &mut self.f
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1> From<X1> for $empty<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: X1) -> Self {
                $empty::new(x)
            }
        }

        // tuple - 2

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2> $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2) {
                (&self.f, &self.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
                (&mut self.f, &mut self.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2> From<(X1, X2)> for $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2)) -> Self {
                $pair::from_fb(x.0, $empty::new(x.1))
            }
        }

        // tuple - 3

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.f, self.b.f, self.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
                (&self.f, &self.b.f, &self.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3> From<(X1, X2, X3)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $empty::new(x.2)))
            }
        }

        // tuple - 4

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $empty::new(x.3))))
            }
        }

        // tuple - 5

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)* X5>>
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
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)* X5>>
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
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $empty::new(x.4)))))
            }
        }

        // tuple - 6

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)* X6>>
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
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)* X6>>
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
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $empty::new(x.5))))))
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
                                $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)* X7>>
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
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
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
                                $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)* X7>>
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
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $empty::new(x.6)))))))
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
                                    $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)* X8>>
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
            /// Converts the queue into its flat tuple representation.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f, &self.b.b.b.b.b.b.b.f)
            }

            /// Returns a flat tuple representation of mutable references to elements in the queue.
            ///
            /// # Examples
            ///
            /// ```ignore
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
            #[inline(always)]
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
                                    $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)* X8>>
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
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $pair::from_fb(x.6, $empty::new(x.7))))))))
            }
        }
    };
}

// # 4. queue-of

#[doc(hidden)]
#[macro_export]
macro_rules! define_queue_of {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        /// Recall that there exist two statically-typed queue (`StQueue`) implementations:
        ///
        /// * `QueueSingle` which includes exactly one element, and
        /// * `Queue` containing multiple (>=2) elements.
        ///
        /// Queues of all lengths can be represented by these two types:
        /// * `QueueSingle<T1>` is a queue with one element,
        /// * `Queue<T1, QueueSingle<T2>>` with two elements,
        /// * `Queue<T1, Queue<T2, QueueSingle<T3>>>` with three elements,
        /// * `Queue<T1, Queue<T2, Queue<T3, QueueSingle<T4>>>>` with four elements,
        /// * and so on, so forth.
        ///
        /// This is possible thanks to generic associated types and recursive type definition of the `Queue`.
        ///
        /// On the other hand, it might make it difficult to hand-write queue types.
        ///
        /// `queue_of` macro is a helper macro to make such type aliasing convenient whenever needed.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// // written with recursive type definition
        /// type Q1 = Queue<i32, Queue<bool, Queue<char, QueueSingle<String>>>>;
        ///
        /// let instance: Q1 = Queue::new(42).push(true).push('x').push("foo".to_string());
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        ///
        /// // alternatively, using with queue_of macro as a flat list
        /// type Q2 = queue_of!(i32, bool, char, String);
        ///
        /// // notice that Q1 and Q2 are aliases for the same type
        /// let instance2: Q2 = instance;
        /// ```
        macro_rules! $queue_of {
            ($t1:ty) => {
                $empty<$($g_lt ,)* $($g ,)* $t1>
            };

            ($t1:ty, $t2:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1, $empty<$($g_lt ,)* $($g ,)* $t2>>
            };

            ($t1:ty, $t2:ty, $t3:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2, $empty<$($g_lt ,)* $($g ,)* $t3>>
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3, $empty<$($g_lt ,)* $($g ,)* $t4>>
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4, $empty<$($g_lt ,)* $($g ,)* $t5>>
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5, $empty<$($g_lt ,)* $($g ,)* $t6>>
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6, $empty<$($g_lt ,)* $($g ,)* $t7>>
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7, $empty<$($g_lt ,)* $($g ,)* $t8>>
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8, $empty<$($g_lt ,)* $($g ,)* $t9>>
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9, $empty<$($g_lt ,)* $($g ,)* $t10>>
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10, $empty<$($g_lt ,)* $($g ,)* $t11>>
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11, $empty<$($g_lt ,)* $($g ,)* $t12>>
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12, $empty<$($g_lt ,)* $($g ,)* $t13>>
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13, $empty<$($g_lt ,)* $($g ,)* $t14>>
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13,
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14, $empty<$($g_lt ,)* $($g ,)* $t15>>
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13,
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14,
                                                                        $pair<$($g_lt ,)* $($g ,)* $t15, $empty<$($g_lt ,)* $($g ,)* $t16>>
                                                                    >
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };
        }
    };
}
