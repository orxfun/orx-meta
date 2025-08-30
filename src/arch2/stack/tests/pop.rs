use crate::stack::{
    empty::EmptyStack,
    stack::{NonEmptyStack, Stack},
};

#[test]
fn one() {
    let x = EmptyStack;
    let x = x.push_back('x');

    assert_eq!(x.back(), &'x');
    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyStack);
}

#[test]
fn two() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);

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
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

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
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

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
