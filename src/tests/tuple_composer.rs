use crate::{Empty, Multi, One, TupleQueue};

#[test]
fn empty() {
    type M = Empty;

    let builder = TupleQueue::<M>::new();
    let value = builder.value();

    assert_eq!(value, ());
}

#[test]
fn one() {
    type M = One<bool>;

    let builder = TupleQueue::<M>::new();
    let builder = builder.add(true);
    let value = builder.value();

    assert_eq!(value, true);
}

#[test]
fn two() {
    type M = Multi<u32, One<bool>>;

    let builder = TupleQueue::<M>::new();
    let builder = builder.add(10);
    let builder = builder.add(true);
    let value = builder.value();

    assert_eq!(value, (10, true));
}

#[test]
fn three() {
    type M = Multi<char, Multi<u32, One<bool>>>;

    let builder = TupleQueue::<M>::new();
    let builder = builder.add('x');
    let builder = builder.add(10);
    let builder = builder.add(true);
    let value = builder.value();

    assert_eq!(value, (('x', 10), true));
}

#[test]
fn four() {
    type M = Multi<String, Multi<char, Multi<u32, One<bool>>>>;

    let builder = TupleQueue::<M>::new();
    let builder = builder.add("xyz".to_string());
    let builder = builder.add('x');
    let builder = builder.add(10);
    let builder = builder.add(true);
    let value = builder.value();

    assert_eq!(value, ((("xyz".to_string(), 'x'), 10), true));
}
