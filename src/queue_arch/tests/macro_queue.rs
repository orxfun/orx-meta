#![allow(dead_code)]

use crate::{
    define_queue_builder_zzz, define_queue_composition, define_queue_core_zzz,
    define_queue_tuple_transformation_zzz,
};

define_queue_core_zzz!(
    names => {
        traits: { queue: Queue, non_empty_queue: NonEmptyQueue },
        structs: { empty: EmptyQueue, single: Single, pair: Pair }
    };
);

// tests

#[test]
fn one() {
    let x = EmptyQueue::new();
    let x = x.push('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn two() {
    let x = EmptyQueue::new();
    let x = x.push('x');
    let x = x.push(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert!(x.is_empty());
}

#[test]
fn three() {
    let x = EmptyQueue::new();
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop();
    assert_eq!(f, String::from("xyz"));

    assert!(x.is_empty());
}

#[test]
fn four() {
    let x = EmptyQueue::new();
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));
    let x = x.push(true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop();
    assert_eq!(f, true);

    assert!(x.is_empty());
}

define_queue_composition!(
    queues => { trait: Queue, empty: EmptyQueue, single: Single, pair: Pair };
    composition => QueueComposition;
);
#[test]
fn composition() {
    // start from empty

    type C = QueueComposition;

    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop();
    assert_eq!(f, true);

    assert!(x.is_empty());

    // start from single
    let x = C::single('x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop();
    assert_eq!(f, true);

    assert!(x.is_empty());
}

define_queue_builder_zzz!(
    queues => { trait: Queue, empty: EmptyQueue, single: Single, pair: Pair };
    builder => Builder;
);
#[test]
fn builder() {
    type Target = Pair<char, Pair<i32, Pair<String, Single<bool>>>>;

    let builder = Builder::<Target, _>::new();

    let builder: Builder<Pair<i32, Pair<String, Single<bool>>>, Single<char>> =
        builder.push('x');

    let builder: Builder<Pair<String, Single<bool>>, Pair<char, Single<i32>>> =
        builder.push(32);

    let builder: Builder<Single<bool>, Pair<char, Pair<i32, Single<String>>>> =
        builder.push("xyz".to_string());

    let builder: Builder<EmptyQueue, Target> = builder.push(true);

    let x = builder.finish();
    assert_eq!(x.len(), 4);

    // or briefly

    let x = Builder::<Target, _>::new()
        .push('x')
        .push(32)
        .push("xyz".to_string())
        .push(true)
        .finish();

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, b) = x.front_back();
    assert_eq!(f, &true);
    assert!(b.is_empty());
    let f = x.into_front();
    assert_eq!(f, true);
}

define_queue_tuple_transformation_zzz!(
    queues => { trait: Queue, empty: EmptyQueue, single: Single, pair: Pair };
);
#[test]
fn tuple() {
    // into tuple
    let x = EmptyQueue::new()
        .push('x')
        .push(32)
        .push(String::from("xyz"))
        .push(true);
    let tuple = x.clone().into_tuple();
    assert_eq!(tuple, ('x', 32, String::from("xyz"), true));

    // from tuple
    assert_eq!(x, tuple.into());
}
