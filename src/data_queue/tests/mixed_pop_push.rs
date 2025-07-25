use crate::data_queue::{
    empty::Empty,
    queue::{NonEmptyQueue, Queue},
};

#[test]
fn mixed_pop_push() {
    let x = Empty
        .push('x')
        .push(32)
        .push(String::from("xyz"))
        .push(true);

    let x = x.pop_front().1.pop_front().1;

    let x = x.push('x').push(32);

    let x = x.pop_front().1; // true, x, 32

    let (f, x) = x.pop_front();
    assert_eq!(f, true);

    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x, Empty);
}
