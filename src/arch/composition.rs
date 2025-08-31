/// A type composition.
pub trait Composition {
    type Empty;

    type One<X>;

    type Multi<X, Y>;
}

pub trait AnotherComposition<L, R> {
    type Composed;
}
