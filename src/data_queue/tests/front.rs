use crate::data_queue::{
    empty::Empty,
    queue::{NonEmptyQueue, Queue},
};

#[test]
fn one() {
    let x = Empty;
    let x = x.push('x');

    assert_eq!(x.front(), &'x');
}

#[test]
fn two() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);

    assert_eq!(x.front(), &'x');
}

#[test]
fn three() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));

    assert_eq!(x.front(), &'x');
}

#[test]
fn four() {
    let x = Empty;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));
    let x = x.push(true);

    assert_eq!(x.front(), &'x');
}
