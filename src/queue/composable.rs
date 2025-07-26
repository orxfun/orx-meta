use crate::{
    Composable,
    queue::{EmptyQueue, Pair, Queue, Single},
};

impl Composable for EmptyQueue {
    type Compose<X> = <Self as Queue>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}

impl<T> Composable for Single<T> {
    type Compose<X> = <Self as Queue>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}

impl<F, B> Composable for Pair<F, B>
where
    B: Queue,
{
    type Compose<X> = <Self as Queue>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}
