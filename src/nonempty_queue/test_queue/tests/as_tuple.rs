use crate::nonempty_queue::test_queue::*;

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
