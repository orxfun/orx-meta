use super::utils::{assert_type, push};
use crate::queue::empty::Empty;

#[test]
fn empty() {
    let x = Empty;

    assert_type(&x, "Empty");
}

#[test]
fn single() {
    let x = Empty;
    let x = push::<_, char>(x);

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);

    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);

    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = push::<_, bool>(x);

    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
