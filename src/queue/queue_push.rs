pub trait QueuePush {
    type PushBack<T>: QueuePush;

    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
