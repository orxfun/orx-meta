use super::utils::assert_type;
use crate::{meta_composition::MetaComposable, meta_queue::empty::Empty};

#[test]
fn single() {
    let x = Empty;
    let x = x.compose::<char>();

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = x.compose::<char>();
    let x = x.compose::<u32>();

    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();

    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();
    let x = x.compose::<bool>();

    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
