use crate::{
    Composable,
    stack::{EmptyStack, Pair, Single, Stack},
};

impl Composable for EmptyStack {
    type Compose<X> = <Self as Stack>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}

impl<T> Composable for Single<T> {
    type Compose<X> = <Self as Stack>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}

impl<F, B> Composable for Pair<F, B>
where
    F: Stack,
{
    type Compose<X> = <Self as Stack>::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}
