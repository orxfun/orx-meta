use crate::{nonempty_queue::*, nonempty_queue_of};

#[test]
fn alias_with_queue_of() {
    let q: nonempty_queue_of!(u32) = QueueSingle::new(42);
    assert_eq!(q.into_tuple(), 42);

    let q: nonempty_queue_of!(u32, char) = QueueSingle::new(42).push('x');
    assert_eq!(q.into_tuple(), (42, 'x'));

    let q: nonempty_queue_of!(u32, char, bool) = QueueSingle::new(42).push('x').push(true);
    assert_eq!(q.into_tuple(), (42, 'x', true));

    let q: nonempty_queue_of!(u32, char, bool, String) = QueueSingle::new(42)
        .push('x')
        .push(true)
        .push("foo".to_string());
    assert_eq!(q.into_tuple(), (42, 'x', true, "foo".to_string()));
}
