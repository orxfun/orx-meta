pub trait MetaComposition: Default {
    type Empty: MetaComposable;

    type Compose<X, Y>: MetaComposable
    where
        X: MetaComposable;

    fn empty() -> Self::Empty {
        Default::default()
    }

    fn compose<X: MetaComposable, Y>(_: X) -> Self::Compose<X, Y> {
        Default::default()
    }
}

pub trait MetaComposable: Default {
    type Compose<X>: MetaComposable;

    fn compose<X>(&self) -> Self::Compose<X> {
        Default::default()
    }
}
