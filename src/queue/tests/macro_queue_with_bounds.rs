#![allow(dead_code)]

use crate::define_queue;

// bounds

pub trait Req {}

define_queue!(
    lifetimes => [];
    generics => [];
    elements => [Req];
    names => {
        traits: {
            queue: Queue,
            non_empty_queue: NonEmptyQueue,
        },
        structs: {
            empty: EmptyQueue,
            single: Single,
            pair: Pair,
            composition: QueueComposition,
            builder: Builder,
        },
    };
);
impl Req for EmptyQueue {}
impl<F: Req> Req for Single<F> {}
impl<F: Req, B: Queue> Req for Pair<F, B> {}

// tests

impl Req for char {}
impl Req for i32 {}
impl Req for String {}
impl Req for bool {}

#[test]
fn one() {
    let x = EmptyQueue::new();
    let x = x.push_back('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn two() {
    let x = EmptyQueue::new();
    let x = x.push_back('x');
    let x = x.push_back(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert!(x.is_empty());
}

#[test]
fn three() {
    let x = EmptyQueue::new();
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.into_front_back();
    assert_eq!(f, String::from("xyz"));

    assert!(x.is_empty());
}

#[test]
fn four() {
    let x = EmptyQueue::new();
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.into_front_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.into_front_back();
    assert_eq!(f, true);

    assert!(x.is_empty());
}

#[test]
fn compose_four() {
    type C = QueueComposition;

    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.into_front_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.into_front_back();
    assert_eq!(f, true);

    assert!(x.is_empty());

    let x = C::single('x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.into_front_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.into_front_back();
    assert_eq!(f, true);

    assert!(x.is_empty());
}

#[test]
fn builder() {
    type Target = Pair<char, Pair<i32, Pair<String, Single<bool>>>>;

    let builder = Builder::<Target, _>::new();

    let builder: Builder<Pair<i32, Pair<String, Single<bool>>>, Single<char>> =
        builder.push_back('x');

    let builder: Builder<Pair<String, Single<bool>>, Pair<char, Single<i32>>> =
        builder.push_back(32);

    let builder: Builder<Single<bool>, Pair<char, Pair<i32, Single<String>>>> =
        builder.push_back("xyz".to_string());

    let builder: Builder<EmptyQueue, Target> = builder.push_back(true);

    let x = builder.finish();

    assert_eq!(x.front(), &'x');
    let (f, x) = x.into_front_back();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.into_front_back();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.into_front_back();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, b) = x.front_back();
    assert_eq!(f, &true);
    assert!(b.is_empty());
    let f = x.into_front();
    assert_eq!(f, true);
}
