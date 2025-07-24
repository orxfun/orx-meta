use crate::{
    composition::Composition,
    queue::from_sequence::*,
    queue::{MetaQueue, empty::Empty},
};

struct TupleComposition;

impl Composition for TupleComposition {
    type Empty = ();

    type One<X> = X;

    type Multi<X, Y> = (X, Y);
}

#[test]
fn empty() {
    type Q = Empty;
    type M = <Q as MetaQueue>::Map<TupleComposition>;

    let _m: M = ();
}

#[test]
fn one() {
    type Q = MetaQueueFrom1<char>;
    type M = <Q as MetaQueue>::Map<TupleComposition>;

    let _m: M = 'x';
}

#[test]
fn two() {
    type Q = MetaQueueFrom2<char, u32>;
    type M = <Q as MetaQueue>::Map<TupleComposition>;

    let _m: M = ('x', 1u32);
}

#[test]
fn three() {
    type Q = MetaQueueFrom3<char, u32, String>;
    type M = <Q as MetaQueue>::Map<TupleComposition>;

    let _m: M = ('x', (1u32, "a".to_string()));
}

#[test]
fn four() {
    type Q = MetaQueueFrom4<char, u32, String, bool>;
    type M = <Q as MetaQueue>::Map<TupleComposition>;

    let _m: M = ('x', (1u32, ("a".to_string(), true)));
}
