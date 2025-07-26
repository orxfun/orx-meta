use crate::stack::{
    empty::EmptyStack,
    stack::{NonEmptyStack, Stack},
};

#[test]
fn one() {
    let x = EmptyStack;
    let x = x.push_back('x');

    assert_eq!(x.back(), &'x');
}

#[test]
fn two() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);

    assert_eq!(x.back(), &32);
}

#[test]
fn three() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.back(), &String::from("xyz"));
}

#[test]
fn four() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    assert_eq!(x.back(), &true);
}
