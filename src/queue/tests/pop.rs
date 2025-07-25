use super::utils::{assert_type, pop_front, push};
use crate::queue::empty::Empty;

#[test]
fn one() {
    let x = Empty;
    let x = push::<_, char>(x);

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    assert_type(&x, "Empty");
}

#[test]
fn two() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);

    let (l, x) = pop_front(x);
    assert_type(&l, "char");

    let (l, x) = pop_front(x);
    assert_type(&l, "u32");

    assert_type(&x, "Empty");
}

#[test]
fn three() {
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);

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
    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = push::<_, bool>(x);

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
