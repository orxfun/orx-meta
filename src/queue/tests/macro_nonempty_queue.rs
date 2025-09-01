use crate::define_non_empty_queue;

define_non_empty_queue!(Queue, MultiQueue, Single, Pair, QueueComposition);

// tests

#[test]
fn one() {
    let x = Single('x');

    assert_eq!(x.front(), &'x');
    let f = x.into_front();
    assert_eq!(f, 'x');
}

#[test]
fn two() {
    let x = Single('x');
    let x = x.push_back(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let f = x.into_front();
    assert_eq!(f, 32);
}

#[test]
fn three() {
    let x = Single('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let f = x.into_front();
    assert_eq!(f, String::from("xyz"));
}

#[test]
fn four() {
    let x = Single('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

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
    let f = x.into_front();
    assert_eq!(f, true);
}

#[test]
fn compose_four() {
    type C = QueueComposition;

    let x = C::single('x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.len(), 4);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    let (f, b) = x.front_back();
    assert_eq!(f, &String::from("xyz"));
    assert_eq!(b.len(), 1);
    assert_eq!(b.front(), &true);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let f = x.into_front();
    assert_eq!(f, true);
}
