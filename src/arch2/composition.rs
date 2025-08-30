use crate::Composable;

pub trait Composition {
    type Empty: Composable;

    fn empty() -> Self::Empty;

    fn compose<X: Composable, Y>(x: X, y: Y) -> <X as Composable>::Compose<Y> {
        x.compose(y)
    }
}
