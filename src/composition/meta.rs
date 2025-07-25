pub trait MetaComposition: Default {
    type Empty: Default;

    type Compose<X, Y>: Default
    where
        X: MetaComposable;

    fn empty() -> Self::Empty {
        Default::default()
    }

    fn compose<X: MetaComposable, Y>(_: X) -> Self::Compose<X, Y> {
        Default::default()
    }
}

pub trait MetaComposable {
    type Compose<X>: Default;

    fn compose<X>(&self) -> Self::Compose<X> {
        Default::default()
    }
}
