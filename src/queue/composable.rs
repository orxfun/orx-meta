use crate::{Composable, queue::Queue};

impl<Q: Queue> Composable for Q {
    type Compose<X> = Q::PushBack<X>;

    fn compose<X>(self, x: X) -> Self::Compose<X> {
        self.push_back(x)
    }
}
