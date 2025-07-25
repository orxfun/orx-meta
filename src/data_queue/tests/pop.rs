use crate::data_queue::{
    empty::EmptyQueue,
    queue::{NonEmptyQueue, Queue},
};

#[test]
fn one() {
    let x = EmptyQueue;
    let x = x.push('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyQueue);
}

#[test]
fn two() {
    let x = EmptyQueue;
    let x = x.push('x');
    let x = x.push(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x, EmptyQueue);
}

#[test]
fn three() {
    let x = EmptyQueue;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x, EmptyQueue);
}

#[test]
fn four() {
    let x = EmptyQueue;
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));
    let x = x.push(true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop_front();
    assert_eq!(f, true);

    assert_eq!(x, EmptyQueue);
}
