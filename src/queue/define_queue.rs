/// The [orx_meta](https://crates.io/crates/orx-meta) crate defines the types for statically typed queues with
/// heterogeneous elements, such as:
/// * [`StQueue`] trait and its two implementations [`EmptyQueue`] and [`Queue`].
/// * [`QueueBuilder`] as a generic builder for any queue, tuple or struct.
/// * [`queue_of`] macro as a helper for creating aliases for complex queue types.
///
/// [`StQueue`]: crate::queue::StQueue
/// [`EmptyQueue`]: crate::queue::EmptyQueue
/// [`Queue`]: crate::queue::Queue
/// [`QueueBuilder`]: crate::queue::QueueBuilder
/// [`queue_of`]: crate::queue_of
///
/// However, the queues are particularly useful when they are used to define compositions of heterogeneous types
/// that exhibit a common behavior; i.e., when all heterogeneous elements in the queue implement a common set of
/// traits.
///
/// It is not possible to day to represent this within the type system. Please see the discussion
/// [here](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md).
///
/// In order to achieve this without any boilerplate, `define_queue` macro can be used.
/// See the corresponding [documentation](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)
/// and corresponding [example](https://github.com/orxfun/orx-meta/blob/main/examples/5_solution_with_macros.rs).
///
/// # Examples
///
/// ## Re-creating the entire queue module of the **orx_meta** crate.
///
/// You may recreate everything defined in the queue module of this crate with the following macro call.
///
/// ```
/// orx_meta::define_queue!(
///     queue => [ StQueue ; EmptyQueue, Queue ];
///     queue_of => queue_of;
///     builder => QueueBuilder;
/// );
/// ```
///
/// * **queue block**
///
/// This is where we give names to the trait, empty queue struct and non-empty queue struct.
/// Above example uses the same names with the crate, but they can be anything.
/// The following is also be a valid block:
///
/// ```rust ignore
/// queue => [ Queue ; EmptyQueue, NonEmptyQueue ];
/// ```
///
/// * **queue_of block**
///
/// Here we simply define the name of the [`queue_of`] macro. The following would also work.
///
/// ```rust ignore
/// queue_of => qof;
/// ```
///
/// * **builder block**
///
/// And similarly, we give a name to our queue builder; again any name would work.
///
/// ```rust ignore
/// builder => MyBuilder;
/// ```
///
/// Note that the `queue_of` and `builder` blocks are optional.
/// If we do not need either of them, we can simply omit the block.
/// The following would give us the most minimalistic implementation of the queue.
///
/// ```rust
/// orx_meta::define_queue!(
///     queue => [ StQueue ; EmptyQueue, Queue ];
/// );
/// ```
///
/// But of course, it would not make sense to re-create the module that already exists.
/// The actual use case of this macro is explained in the following section.
///
/// ## Creating custom own queue module where elements share a common behavior
///
/// Consider the example define [here](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md).
///
/// We want a statically typed queue of heterogeneous elements all of which implement the `Draw` trait.
///
/// In this case, we can use the additional **elements** block where we define a comma-separated list of traits.
///
/// ```rust ignore
/// elements => [Draw];
/// ```
///
/// This ensures that we can only push elements of types implementing these traits to the queue.
/// Further, our two queue implementations must implement these traits representing the identity and composition
/// behavior.
///
/// The entire implementation of the queues, type helper macro and the builder specific to to the `Draw` behavior
/// can be conveniently generated with the `define_queue` macro as follows:
///
/// ```
/// pub trait Draw {
///     fn draw(&self);
/// }
///
/// pub struct Button;
/// impl Draw for Button {
///     fn draw(&self) {
///         println!("Button")
///     }
/// }
///
/// pub struct SelectBox;
/// impl Draw for SelectBox {
///     fn draw(&self) {
///         println!("SelectBox")
///     }
/// }
///
/// orx_meta::define_queue!(
///     elements => [Draw];
///     queue => [ StDrawQueue ; EmptyDrawQueue, DrawQueue ];
///     queue_of => queue_of;
///     builder => DrawQueueBuilder;
/// );
/// impl Draw for EmptyDrawQueue {
///     // identity behavior: do nothing
///     fn draw(&self) {}
/// }
/// impl<F: Draw, B: StDrawQueue> Draw for DrawQueue<F, B> {
///     // composition behavior: draw them both
///     fn draw(&self) {
///         self.f.draw();
///         self.b.draw();
///     }
/// }
///
/// let screen = EmptyDrawQueue::new()
///     .push(Button)
///     .push(Button)
///     .push(SelectBox)
///     .push(Button)
///     .push(SelectBox);
/// screen.draw(); // draw all components
/// ```
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

        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
        $crate::define_queue_tuple_transformation!(
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
            fn push<T>(self, x: T) -> Self::PushBack<T>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;


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
        /// ```
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
                &self.b
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
        /// A type-safe builder for queues such that:
        ///
        /// * `push` can only be called correct number of times with correct types,
        /// * `finish` can only be called after all elements are pushed.
        ///
        /// Further, since queues can represent ad-hoc structs, `QueueBuilder` can
        /// be used as a generic builder for any struct or tuple.
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
        /// type MyQueue = Queue<u32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>>;
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
        /// This provides a convenient way to build complex types without errors while getting compiler support on what to push next.
        /// However, it is not easy to hand-write the type alias for the complex recursive queue type.
        /// Therefore, this builder pattern is most useful when used together with the [`queue_of`] macro.
        /// The above example could be re-written as follows with the `queue_of` macro.
        ///
        /// [`queue_of`]: crate::queue_of
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
        pub struct $builder<$($g_lt ,)* $($g ,)* Remaining, Current = $empty<$($g_lt ,)* $($g ,)*>>
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
            /// let builder = QueueBuilder::<EmptyQueue>::new();
            /// let instance = builder.finish();
            /// assert_eq!(instance, EmptyQueue);
            ///
            /// let builder = QueueBuilder::<queue_of!(u32)>::new();
            /// let instance = builder.push(42).finish();
            /// assert_eq!(instance.as_tuple(), (&42));
            ///
            /// let builder = QueueBuilder::<queue_of!(u32, bool)>::new();
            /// let instance = builder.push(42).push(true).finish();
            /// assert_eq!(instance.as_tuple(), (&42, &true));
            ///
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

// # 3. tuple

#[doc(hidden)]
#[macro_export]
macro_rules! define_queue_tuple_transformation {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        // tuple - 1

        #[allow(dead_code)]
        impl<$($g_lt ,)* X1, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)*>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
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
            pub fn into_tuple(self) -> X1 {
                self.f
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
                &self.f
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
                &mut self.f
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1> From<X1> for $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)*>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: X1) -> Self {
                $pair::new(x)
            }
        }

        // tuple - 2

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2> $pair<$($g_lt ,)* $($g ,)* X1, $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)*>>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
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
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
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
                (&self.f, &self.b.f)
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
                (&mut self.f, &mut self.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2> From<(X1, X2)> for $pair<$($g_lt ,)* $($g ,)* X1, $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)*>>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2)) -> Self {
                $pair::from_fb(x.0, $pair::new(x.1))
            }
        }

        // tuple - 3

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f)
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
                (&mut self.f, &mut self.b.f, &mut self.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3> From<(X1, X2, X3)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)*>>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::new(x.2)))
            }
        }

        // tuple - 4

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f)
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
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)*>>>
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
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::new(x.3))))
            }
        }

        // tuple - 5

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f)
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
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)*>>>
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
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::new(x.4)))))
            }
        }

        // tuple - 6

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f)
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
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)*>>>
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
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::new(x.5))))))
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
                                $pair<$($g_lt ,)* $($g ,)* X6, $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f)
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
                                $pair<$($g_lt ,)* $($g ,)* X6, $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)*>>>
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
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $pair::new(x.6)))))))
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
                                    $pair<$($g_lt ,)* $($g ,)* X7, $pair<$($g_lt ,)* $($g ,)* X8, $empty<$($g_lt ,)* $($g ,)*>>>
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
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
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
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f, &self.b.b.b.b.b.b.b.f)
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
                                    $pair<$($g_lt ,)* $($g ,)* X7, $pair<$($g_lt ,)* $($g ,)* X8, $empty<$($g_lt ,)* $($g ,)*>>>
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
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $pair::from_fb(x.6, $pair::new(x.7))))))))
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
        /// Note that any statically-typed queue of heterogeneous elements can be represented with two [`StQueue`] implementations:
        ///
        /// * [`EmptyQueue`], and
        /// * a non-empty [`Queue`].
        ///
        /// This is possible thanks to generic associated types and recursive type definition of the `Queue`.
        ///
        /// On the other hand, it might make it difficult to hand-write the specific queue types.
        ///
        /// Consider for instance the type of a queue containing four elements of types `i32`, `bool`, `char` and `String`.
        ///
        /// The type of this queue would be:
        ///
        /// ```
        /// use orx_meta::queue::*;
        ///
        /// type MyQueue = Queue<i32, Queue<bool, Queue<char, Queue<String, EmptyQueue>>>>;
        ///
        /// let instance: MyQueue = Queue::new(42).push(true).push('x').push("foo".to_string());
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        ///
        /// Notice that the nested type definition can get complicated as our queue contains more and more fields.
        ///
        /// `queue_of` macro is a helper macro to make such type aliasing convenient as follows:
        ///
        /// ```
        /// use orx_meta::queue::*;
        /// use orx_meta::queue_of;
        ///
        /// type MyQueue = queue_of!(i32, bool, char, String);
        ///
        /// let instance: MyQueue = Queue::new(42).push(true).push('x').push("foo".to_string());
        /// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
        /// ```
        ///
        /// [`StQueue`]: crate::queue::StQueue
        /// [`EmptyQueue`]: crate::queue::EmptyQueue
        /// [`Queue`]: crate::queue::Queue
        macro_rules! $queue_of {
            () => {
                $empty<$($g_lt ,)* $($g ,)*>
            };

            ($t1:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1, $empty<$($g_lt ,)* $($g ,)*>>
            };

            ($t1:ty, $t2:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $empty<$($g_lt ,)* $($g ,)*>>>
            };

            ($t1:ty, $t2:ty, $t3:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $empty<$($g_lt ,)* $($g ,)*>>>
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3, $pair<$($g_lt ,)* $($g ,)* $t4, $empty<$($g_lt ,)* $($g ,)*>>>
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4, $pair<$($g_lt ,)* $($g ,)* $t5, $empty<$($g_lt ,)* $($g ,)*>>>
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5, $pair<$($g_lt ,)* $($g ,)* $t6, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                    $pair<$($g_lt ,)* $($g ,)* $t6, $pair<$($g_lt ,)* $($g ,)* $t7, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                        $pair<$($g_lt ,)* $($g ,)* $t7, $pair<$($g_lt ,)* $($g ,)* $t8, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                            $pair<$($g_lt ,)* $($g ,)* $t8, $pair<$($g_lt ,)* $($g ,)* $t9, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                $pair<$($g_lt ,)* $($g ,)* $t9, $pair<$($g_lt ,)* $($g ,)* $t10, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                    $pair<$($g_lt ,)* $($g ,)* $t10, $pair<$($g_lt ,)* $($g ,)* $t11, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                        $pair<$($g_lt ,)* $($g ,)* $t11, $pair<$($g_lt ,)* $($g ,)* $t12, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                            $pair<$($g_lt ,)* $($g ,)* $t12, $pair<$($g_lt ,)* $($g ,)* $t13, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                                $pair<$($g_lt ,)* $($g ,)* $t13, $pair<$($g_lt ,)* $($g ,)* $t14, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14, $pair<$($g_lt ,)* $($g ,)* $t15, $empty<$($g_lt ,)* $($g ,)*>>>
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
                                                                        $pair<$($g_lt ,)* $($g ,)* $t15, $pair<$($g_lt ,)* $($g ,)* $t16, $empty<$($g_lt ,)* $($g ,)*>>>
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
