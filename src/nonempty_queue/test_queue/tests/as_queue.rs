use crate::nonempty_queue::test_queue::{Queue, QueueSingle, StQueue};

#[test]
fn push_and_pop() {
    let q = QueueSingle::new(42);
    assert_eq!(q.len(), 1);
    assert_eq!(q.as_tuple(), &42);

    let q = q.push('x');
    assert_eq!(q.len(), 2);
    assert_eq!(q.as_tuple(), (&42, &'x'));

    let q = q.push(true);
    assert_eq!(q.len(), 3);
    assert_eq!(q.as_tuple(), (&42, &'x', &true));

    let q = q.push("foo".to_string());
    assert_eq!(q.len(), 4);
    assert_eq!(q.as_tuple(), (&42, &'x', &true, &"foo".to_string()));

    let (elem, q) = q.pop();
    assert_eq!(elem, 42);
    assert_eq!(q.len(), 3);
    assert_eq!(q.as_tuple(), (&'x', &true, &"foo".to_string()));

    let (elem, q) = q.pop();
    assert_eq!(elem, 'x');
    assert_eq!(q.len(), 2);
    assert_eq!(q.as_tuple(), (&true, &"foo".to_string()));

    let (elem, q) = q.pop();
    assert_eq!(elem, true);
    assert_eq!(q.len(), 1);
    assert_eq!(q.as_tuple(), &"foo".to_string());

    let elem = q.into_front();
    assert_eq!(elem, "foo".to_string());
}

#[test]
fn front_back() {
    let q = QueueSingle::new(42);
    assert_eq!(q.front(), &42);
}
