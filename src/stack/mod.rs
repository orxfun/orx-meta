#[cfg(test)]
mod tests;

mod composable;
mod composition;
mod empty;
mod pair;
mod single;
mod stack;

pub use composition::StackComposition;
pub use empty::EmptyStack;
pub use pair::Pair;
pub use single::Single;
pub use stack::{NonEmptyStack, Stack};
