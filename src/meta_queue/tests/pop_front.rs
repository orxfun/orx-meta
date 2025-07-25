use super::utils::{assert_type, pop_front};
use crate::{
    composition::{MetaComposable, MetaComposition},
    meta_queue::composition::MetaQueueComposition,
};

type C = MetaQueueComposition;

#[test]
fn one() {
    let x = C::single::<char>();

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    assert_type(&x, "Empty");
}

#[test]
fn two() {
    let x = C::pair::<char, u32>();

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    let (l, x) = pop_front(x);
    assert_type(&l, "u32");

    assert_type(&x, "Empty");
}

#[test]
fn three() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    let (l, x) = pop_front(x);
    assert_type(&l, "u32");

    let (l, x) = pop_front(x);
    assert_type(&l, "String");

    assert_type(&x, "Empty");
}

#[test]
fn four() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();
    let x = x.compose::<bool>();

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    let (l, x) = pop_front(x);
    assert_type(&l, "u32");

    let (l, x) = pop_front(x);
    assert_type(&l, "String");

    let (l, x) = pop_front(x);
    assert_type(&l, "bool");

    assert_type(&x, "Empty");
}
