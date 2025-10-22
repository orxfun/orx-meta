use crate::queue::{QueueSingle, StQueue};
use core::marker::PhantomData;

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
/// ```
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
/// ```
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
/// ```compile_fail
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
/// ```compile_fail
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
pub struct QueueBuilder<Target>
where
    Target: StQueue,
{
    target: PhantomData<Target>,
}

impl<Target> Default for QueueBuilder<Target>
where
    Target: StQueue,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Target> QueueBuilder<Target>
where
    Target: StQueue,
{
    /// Creates a new empty builder for the `Target` type defined as the generic argument.
    pub fn new() -> Self {
        Self {
            target: Default::default(),
        }
    }

    /// Pushes the next `element` to build the target type.
    pub fn push(
        self,
        element: Target::Front,
    ) -> QueueBuilding<Target, Target::Back, QueueSingle<Target::Front>> {
        QueueBuilding::new(QueueSingle::new(element))
    }
}

pub struct QueueBuilding<Target, Remaining, Current>
where
    Target: StQueue,
    Remaining: StQueue,
    Current: StQueue,
{
    target: PhantomData<Target>,
    remaining: PhantomData<Remaining>,
    current: Current,
}

impl<Target, Remaining, Current> QueueBuilding<Target, Remaining, Current>
where
    Target: StQueue,
    Remaining: StQueue,
    Current: StQueue,
{
    fn new(current: Current) -> Self {
        Self {
            target: Default::default(),
            remaining: Default::default(),
            current,
        }
    }

    /// Pushes the next `element` to build the target type.
    pub fn push(
        self,
        element: Remaining::Front,
    ) -> QueueBuilding<Target, Remaining::Back, Current::PushBack<Remaining::Front>> {
        QueueBuilding::new(self.current.push(element))
    }

    /// Completes the builder and returns the built target type.
    pub fn finish(self) -> Current
    where
        Target: StQueue<Front = Current::Front, Back = Current::Back>,
    {
        self.current
    }
}
