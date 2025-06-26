use crate::data_composer::DataComposer;

/// A data composer where the composition of data pieces `a` and `b`
/// is the tuple `(a, b)`.
#[derive(Default)]
pub struct TupleComposer;

impl DataComposer for TupleComposer {
    type Compose<X, Y> = (X, Y);

    fn compose<X, Y>(x: X, y: Y) -> Self::Compose<X, Y> {
        (x, y)
    }
}
