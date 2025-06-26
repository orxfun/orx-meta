/// Trait defining composition of two pieces of data.
pub trait DataComposer: Default {
    /// Type resulting from composing data of type `X` with data of type `Y`.
    type Compose<X, Y>;

    /// Returns data obtained by composing `x` and `y`.
    fn compose<X, Y>(x: X, y: Y) -> Self::Compose<X, Y>;
}
