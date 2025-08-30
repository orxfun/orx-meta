use crate::{
    Composable,
    queue::{EmptyQueue, NonEmptyQueue},
};

#[test]
fn one() {
    let x = EmptyQueue;
    let x = x.compose('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyQueue);
}

#[test]
fn two() {
    let x = EmptyQueue;
    let x = x.compose('x');
    let x = x.compose(32);

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
    let x = x.compose('x');
    let x = x.compose(32);
    let x = x.compose(String::from("xyz"));

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
    let x = x.compose('x');
    let x = x.compose(32);
    let x = x.compose(String::from("xyz"));
    let x = x.compose(true);

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
