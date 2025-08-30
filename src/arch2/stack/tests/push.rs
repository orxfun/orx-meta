use crate::stack::{empty::EmptyStack, stack::Stack};

#[test]
fn empty() {
    let x = EmptyStack;

    assert_eq!(x.len(), 0);
}

#[test]
fn one() {
    let x = EmptyStack;
    let x = x.push_back('x');

    assert_eq!(x.len(), 1);
}

#[test]
fn two() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);

    assert_eq!(x.len(), 2);
}

#[test]
fn three() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.len(), 3);
}

#[test]
fn four() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    assert_eq!(x.len(), 4);
}
