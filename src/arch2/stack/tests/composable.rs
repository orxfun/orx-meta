use crate::{
    Composable,
    stack::{EmptyStack, NonEmptyStack},
};

#[test]
fn one() {
    let x = EmptyStack;
    let x = x.compose('x');

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyStack);
}

#[test]
fn two() {
    let x = EmptyStack;
    let x = x.compose('x');
    let x = x.compose(32);

    assert_eq!(x.back(), &32);
    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyStack);
}

#[test]
fn three() {
    let x = EmptyStack;
    let x = x.compose('x');
    let x = x.compose(32);
    let x = x.compose(String::from("xyz"));

    assert_eq!(x.back(), &String::from("xyz"));
    let (f, x) = x.pop_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.back(), &32);
    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyStack);
}

#[test]
fn four() {
    let x = EmptyStack;
    let x = x.compose('x');
    let x = x.compose(32);
    let x = x.compose(String::from("xyz"));
    let x = x.compose(true);

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

    assert_eq!(x, EmptyStack);
}
