/// A queue of types with push and pop operations:
///
/// * pushes the type to the back, and
/// * pops the type from the front,
///
/// and hence, allows first-in-first-out (FIFO) traversal.
pub trait MetaQueue: Default {
    /// The queue obtained by pushing the type `X` to the back of this queue.
    type Push<X>: MetaQueue;

    /// The queue containing zero [`Empty`] or one type [`One`] in the front of this queue;
    /// i.e., the queue obtained by popping one element from the front of this queue.
    ///
    /// [`Empty`]: crate::Empty
    /// [`One`]: crate::One
    type Front: MetaQueue;

    /// The queue succeeding the `Front`; i.e., the remaining queue after popping one
    /// element from the front of this queue.
    type Back: MetaQueue;
}
