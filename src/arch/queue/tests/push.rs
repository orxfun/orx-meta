use super::utils::{assert_type, push};
use crate::queue::empty::Empty;

#[test]
fn empty() {
    let x = Empty;

    assert_type(&x, "Empty");
}

#[test]
fn one() {
    let x = Empty;
    let x = push::<_, char>(x);

    assert_type(&x, "One<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);

    assert_type(&x, "Multi<char,One<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);

    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = push::<_, bool>(x);

    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");
}
