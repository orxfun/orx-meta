pub trait Composition {
    type Empty: Default;

    type Single<X>;

    type Compose<X, Y>;

    fn compose<X, Y>(x: X, y: Y) -> Self::Compose<X, Y>;
}
