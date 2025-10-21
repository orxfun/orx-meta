use crate::{
    nonempty_queue::test_queue::{Queue, QueueSingle, StQueue},
    nonempty_queue_of,
};

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

    let elem = q.into_inner();
    assert_eq!(elem, "foo".to_string());
}

#[test]
fn into_tuple() {
    let q = QueueSingle::new(42);
    assert_eq!(q.into_tuple(), 42);

    let q = QueueSingle::new(42).push('x');
    assert_eq!(q.into_tuple(), (42, 'x'));

    let q = QueueSingle::new(42).push('x').push(true);
    assert_eq!(q.into_tuple(), (42, 'x', true));

    let q = QueueSingle::new(42)
        .push('x')
        .push(true)
        .push("foo".to_string());
    assert_eq!(q.into_tuple(), (42, 'x', true, "foo".to_string()));
}

#[test]
fn as_mut_tuple() {
    let mut q = QueueSingle::new(42);
    let a = q.as_tuple_mut();
    *a += 1;
    assert_eq!(q.as_tuple(), &43);

    let mut q = QueueSingle::new(42).push('x');
    let (a, b) = q.as_tuple_mut();
    *a += 1;
    *b = 'y';
    assert_eq!(q.as_tuple(), (&43, &'y'));

    let mut q = QueueSingle::new(42).push('x').push(true);
    let (a, b, c) = q.as_tuple_mut();
    *a += 1;
    *b = 'y';
    *c = false;
    assert_eq!(q.as_tuple(), (&43, &'y', &false));

    let mut q = QueueSingle::new(42)
        .push('x')
        .push(true)
        .push("foo".to_string());
    let (a, b, c, d) = q.as_tuple_mut();
    *a += 1;
    *b = 'y';
    *c = false;
    *d = "bar".to_string();
    assert_eq!(q.as_tuple(), (&43, &'y', &false, &"bar".to_string()));
}

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
