use super::utils::assert_type;
use crate::{composition::MetaComposition, meta_queue::composition::MetaQueueComposition};

type C = MetaQueueComposition;

#[test]
fn empty() {
    let x = C::empty();

    assert_type(&x, "Empty");
}

#[test]
fn single() {
    let x = C::empty();
    let x = C::compose::<_, char>(x);

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = C::empty();
    let x = C::compose::<_, char>(x);
    let x = C::compose::<_, u32>(x);

    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = C::empty();
    let x = C::compose::<_, char>(x);
    let x = C::compose::<_, u32>(x);
    let x = C::compose::<_, String>(x);

    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = C::empty();
    let x = C::compose::<_, char>(x);
    let x = C::compose::<_, u32>(x);
    let x = C::compose::<_, String>(x);
    let x = C::compose::<_, bool>(x);

    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
