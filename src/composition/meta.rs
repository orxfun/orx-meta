pub trait Composition {
    type Empty;

    type Single<X>;

    type Pair<X, Y>;

    type Trio<X, Y, Z>;

    fn empty() -> Self::Empty;

    fn single<X>(x: X) -> Self::Single<X>;

    fn pair<X, Y>(x: X, y: Y) -> Self::Pair<X, Y>;

    fn compose_with_single<X, Y>(single: Self::Single<X>, y: Y) -> Self::Pair<X, Y>;

    fn compose_with_pair<X, Y, Z>(pair: Self::Pair<X, Y>, z: Z) -> Self::Trio<X, Y, Z>;
}
