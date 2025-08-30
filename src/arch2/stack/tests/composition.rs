use crate::{
    Composition,
    stack::{NonEmptyStack, Stack, StackComposition},
};

type C = StackComposition;

#[test]
fn one() {
    let x = C::empty();
    let x = C::compose(x, 'x');

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn two() {
    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);

    assert_eq!(x.back(), &32);
    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn three() {
    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));

    assert_eq!(x.back(), &String::from("xyz"));
    let (f, x) = x.pop_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.back(), &32);
    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn four() {
    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.back(), &true);
    let (f, x) = x.pop_back();
    assert_eq!(f, true);

    assert_eq!(x.back(), &String::from("xyz"));
    let (f, x) = x.pop_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.back(), &32);
    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}
