/// Trait defining composition of two pieces of data.
pub trait DataComposer: Default {
    /// Type of empty data.
    type Empty;

    /// Type of a single data piece of type `X`.
    type Singleton<X>;

    /// Type resulting from composing data of type `X` with data of type `Y`.
    type Compose<X, Y>;

    /// Returns empty data.
    fn empty() -> Self::Empty;

    /// Returns the single data piece `x`.
    fn singleton<X>(x: X) -> Self::Singleton<X>;

    /// Returns data obtained by composing `x` and `y`.
    fn compose<X, Y>(x: X, y: Y) -> Self::Compose<X, Y>;
}
