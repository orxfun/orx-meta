use crate::{Composition, stack::EmptyStack};

#[derive(Default)]
pub struct StackComposition;

impl Composition for StackComposition {
    type Empty = EmptyStack;

    fn empty() -> Self::Empty {
        EmptyStack
    }
}
