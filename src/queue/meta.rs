/// A strongly typed queue of elements of arbitrary types.
///
/// There exist two implementations:
/// * `EmptyQueue`
/// * `Queue<F, B>`: a non-empty queue where the front element is of type `F`
///   and the remaining elements is a queue of type `B`.
pub trait QueueMeta {
    type PushBack<T>: QueueMeta;

    type Front;

    type Back: QueueMeta;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
