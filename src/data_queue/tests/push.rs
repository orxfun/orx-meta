use crate::data_queue::{empty::Empty, queue::Queue};

#[test]
fn empty() {
    let x = Empty;

    assert_eq!(x.len(), 0);
}

#[test]
fn one() {
    let x = Empty;
    let x = x.push('x');

    assert_eq!(x.len(), 1);
}

#[test]
fn two() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);

    assert_eq!(x.len(), 2);
}

#[test]
fn three() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));

    assert_eq!(x.len(), 3);
}

#[test]
fn four() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));
    let x = x.push(true);

    assert_eq!(x.len(), 4);
}
