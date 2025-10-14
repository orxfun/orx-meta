use crate::product::{push::Push, single::Single};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Empty;

impl Push for Empty {
    type PushBack<X> = Single<X>;

    type PushFront<X> = Single<X>;

    const LEN: usize = 0;

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        Single::new(element)
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        Single::new(element)
    }
}
