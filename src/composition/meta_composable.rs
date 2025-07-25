pub trait MetaComposable: Default {
    type Compose<X>: MetaComposable;

    fn compose<X>(&self) -> Self::Compose<X> {
        Default::default()
    }
}
