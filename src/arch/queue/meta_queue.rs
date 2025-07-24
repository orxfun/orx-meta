use crate::composition::Composition;

/// A queue of types with push and pop operations:
///
/// * pushes the type to the back, and
/// * pops the type from the front,
///
/// and hence, allows first-in-first-out (FIFO) traversal.
pub trait MetaQueue: Default {
    /// The queue obtained by pushing the type `X` to the back of this queue.
    type Push<X>: MetaQueue;

    /// The queue obtained by extending this queue by appending the other queue `X` to the back.
    type Extend<X>: MetaQueue
    where
        X: MetaQueue;

    /// Type in the front of the queue.
    ///
    /// When it is [`Never`], it marks the end of the queue.
    ///
    /// [`Never`]: crate::Never
    type Front;

    /// The queue succeeding the `Front`; i.e., the remaining queue after popping one
    /// element from the front of this queue.
    type Back: MetaQueue;

    type Map<C: Composition>;
}
