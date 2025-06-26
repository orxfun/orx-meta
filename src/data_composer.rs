/// Trait defining composition of two pieces of data.
pub trait DataComposer: Default {
    /// Type of empty data.
    type Empty;

    /// Type of a single data piece of type `X`.
    type One<X>;

    /// Type resulting from composing data of type `X` with data of type `Y`.
    type Multi<X, Y>;

    /// Returns empty data.
    fn empty() -> Self::Empty;

    /// Returns the single data piece `x`.
    fn one<X>(x: X) -> Self::One<X>;

    fn one_to_multi<X, Y>(x: Self::One<X>, y: Y) -> Self::Multi<X, Y>;

    fn multi_to_multi<X, Y, Z>(xy: Self::Multi<X, Y>, z: Z) -> Self::Multi<Self::Multi<X, Y>, Z>;
}
