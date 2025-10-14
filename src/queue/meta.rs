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
