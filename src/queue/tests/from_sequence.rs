use super::utils::assert_type;
use crate::queue::from_sequence::*;

#[test]
fn empty() {
    let x = MetaQueueOf0::default();

    assert_type(&x, "Empty");
}

#[test]
fn one() {
    let x = MetaQueueOf1::<char>::default();

    assert_type(&x, "One<char>");
}

#[test]
fn two() {
    let x = MetaQueueOf2::<char, u32>::default();

    assert_type(&x, "Multi<char,One<u32>>");
}

#[test]
fn three() {
    let x = MetaQueueOf3::<char, u32, String>::default();
    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");
}

#[test]
fn four() {
    let x = MetaQueueOf4::<char, u32, String, bool>::default();

    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");
}

#[test]
fn five() {
    let x = MetaQueueOf5::<char, u32, String, bool, u8>::default();

    assert_type(
        &x,
        "Multi<char,Multi<u32,Multi<String,Multi<bool,One<u8>>>>>",
    );
}

#[test]
fn six() {
    let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();

    assert_type(
        &x,
        "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
    );
}

#[test]
fn seven() {
    let x = MetaQueueOf7::<char, u32, String, bool, u8, f64, ()>::default();

    assert_type(
        &x,
        "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,Multi<f64,One<()>>>>>>>",
    );
}

#[test]
fn eight() {
    let x = MetaQueueOf8::<char, u32, String, bool, u8, f64, (), Vec<usize>>::default();

    assert_type(
        &x,
        "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,Multi<f64,Multi<(),One<Vec<usize>>>>>>>>>",
    );
}
