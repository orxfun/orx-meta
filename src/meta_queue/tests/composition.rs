use super::utils::assert_type;
use crate::{
    composition::{MetaComposable, MetaComposition},
    meta_queue::composition::MetaQueueComposition,
};

type C = MetaQueueComposition;

#[test]
fn empty() {
    let x = C::empty();

    assert_type(&x, "Empty");
}

#[test]
fn single() {
    let x = C::single::<char>();
    assert_type(&x, "Single<char>");

    let x = C::empty();
    let x = x.compose::<char>();

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = C::pair::<char, u32>();
    assert_type(&x, "Pair<char,Single<u32>>");

    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();

    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();

    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();
    let x = x.compose::<bool>();

    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
