pub trait QueueMeta {
    type PushBack<T>: QueueMeta;

    type Front;

    type Back: QueueMeta;

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
