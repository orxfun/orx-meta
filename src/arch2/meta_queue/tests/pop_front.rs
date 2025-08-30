use super::utils::assert_type;
use crate::{
    meta_composition::{MetaComposable, MetaComposition},
    meta_queue::{composition::MetaQueueComposition, queue::MetaQueue},
};

type C = MetaQueueComposition;

#[test]
fn one() {
    let x = C::single::<char>();

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<char>");

    assert_type(&x, "Empty");
}

#[test]
fn two() {
    let x = C::pair::<char, u32>();

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<char>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<u32>");

    assert_type(&x, "Empty");
}

#[test]
fn three() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<char>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<u32>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<String>");

    assert_type(&x, "Empty");
}

#[test]
fn four() {
    let x = C::empty();
    let x = x.compose::<char>();
    let x = x.compose::<u32>();
    let x = x.compose::<String>();
    let x = x.compose::<bool>();

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<char>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<u32>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<String>");

    let (f, x) = x.pop_front();
    assert_type(&f, "Single<bool>");

    assert_type(&x, "Empty");
}
