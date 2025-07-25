use super::utils::assert_type;
use crate::meta_queue::{empty::Empty, queue::MetaQueue};

#[test]
fn empty() {
    let x = Empty;

    assert_type(&x, "Empty");
}

#[test]
fn single() {
    let x = Empty;
    let x = x.push::<char>();

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = x.push::<char>();
    let x = x.push::<u32>();

    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = x.push::<char>();
    let x = x.push::<u32>();
    let x = x.push::<String>();

    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = x.push::<char>();
    let x = x.push::<u32>();
    let x = x.push::<String>();
    let x = x.push::<bool>();

    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
