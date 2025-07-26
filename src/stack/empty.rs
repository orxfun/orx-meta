use crate::{Never, stack::single::Single, stack::stack::Stack};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyStack;

impl Stack for EmptyStack {
    type PushBack<X> = Single<X>;

    type Front = EmptyStack;

    type Back = Never;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}
