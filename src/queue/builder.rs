use crate::queue::{EmptyQueue, st_queue::StQueue};
use core::marker::PhantomData;

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
pub struct QueueBuilder<Remaining, Current = EmptyQueue>
where
    Remaining: StQueue,
    Current: StQueue,
{
    current: Current,
    remaining: PhantomData<Remaining>,
}

impl<Remaining> QueueBuilder<Remaining, EmptyQueue>
where
    Remaining: StQueue,
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
            current: EmptyQueue,
            remaining: Default::default(),
        }
    }
}

impl<Remaining> Default for QueueBuilder<Remaining, EmptyQueue>
where
    Remaining: StQueue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Remaining, Current> QueueBuilder<Remaining, Current>
where
    Remaining: StQueue,
    Current: StQueue,
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
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilder<Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilder {
            current: self.current.push(element),
            remaining: Default::default(),
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
        Remaining: StQueue<Back = Remaining>,
    {
        self.current
    }
}
