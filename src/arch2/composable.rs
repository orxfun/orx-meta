pub trait Composable {
    type Compose<X>: Composable;

    fn compose<X>(self, x: X) -> Self::Compose<X>;
}
