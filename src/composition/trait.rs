pub trait Composition {
    type Empty;

    type Single<X>;

    type Pair<X, Y>;

    fn empty() -> Self::Empty;

    fn single<X>(x: X) -> Self::Single<X>;

    fn pair<X, Y>(x: X, y: Y) -> Self::Pair<X, Y>;
}
