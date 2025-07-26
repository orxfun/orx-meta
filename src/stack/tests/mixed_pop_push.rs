use crate::stack::{
    empty::EmptyStack,
    stack::{NonEmptyStack, Stack},
};

#[test]
fn mixed_pop_push() {
    let x = EmptyStack
        .push_back('x')
        .push_back(32)
        .push_back(String::from("xyz"))
        .push_back(true);

    let x = x.pop_back().1.pop_back().1;

    let x = x.push_back('x').push_back(32);

    let x = x.pop_back().1; // x, 32, x

    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    let (f, x) = x.pop_back();
    assert_eq!(f, 32);

    let (f, x) = x.pop_back();
    assert_eq!(f, 'x');

    assert_eq!(x, EmptyStack);
}
