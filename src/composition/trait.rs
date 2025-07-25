pub trait Composition {
    type Empty;

    type Single<X>;

    type Pair<X, Y>;
}
