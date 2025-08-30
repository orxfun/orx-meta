use crate::meta_queue::single::Single;

/// A queue of types with push and pop operations:
///
/// * pushes the type to the back, and
/// * pops the type from the front,
///
/// and hence, allows first-in-first-out (FIFO) traversal.
pub trait MetaQueue: Default {
    /// The queue obtained by pushing the type `X` to the back of this queue.
    type Push<X>: MetaQueue;

    /// Type in the front of the queue.
    ///
    /// When it is [`Never`], it marks the end of the queue.
    ///
    /// [`Never`]: crate::Never
    type Front;

    /// The queue succeeding the `Front`; i.e., the remaining queue after popping one
    /// element from the front of this queue.
    type Back: MetaQueue;

    fn push<X>(self) -> Self::Push<X> {
        Default::default()
    }

    fn pop_front(self) -> (Single<Self::Front>, Self::Back) {
        Default::default()
    }
}
