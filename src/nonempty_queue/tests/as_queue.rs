use crate::nonempty_queue::{QueueSingle, StQueue};

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
fn front_access() {
    let mut q = QueueSingle::new(42);
    *q.front_mut() += 1;
    assert_eq!(q.front(), &43);
    assert_eq!(q.into_front(), 43);

    let mut q = QueueSingle::new(42).push('x');
    *q.front_mut() += 1;
    assert_eq!(q.front(), &43);
    assert_eq!(q.into_front(), 43);

    let mut q = QueueSingle::new(42).push('x').push(true);
    *q.front_mut() += 1;
    assert_eq!(q.front(), &43);
    assert_eq!(q.into_front(), 43);
}

#[test]
fn back_access() {
    let mut q = QueueSingle::new(42).push('x');
    *q.front_mut() += 1;
    *q.back_mut().front_mut() = 'y';
    assert_eq!(q.front(), &43);
    assert_eq!(q.back().front(), &'y');
    assert_eq!(q.into_back().into_front(), 'y');

    let mut q = QueueSingle::new(42).push('x').push(true);
    *q.front_mut() += 1;
    *q.back_mut().front_mut() = 'y';
    *q.back_mut().back_mut().front_mut() = false;
    assert_eq!(q.front(), &43);
    assert_eq!(q.back().front(), &'y');
    assert_eq!(q.back().back().front(), &false);
    assert_eq!(q.into_back().into_back().into_front(), false);
}

#[test]
fn front_back_mut() {
    let mut q = QueueSingle::new(42).push('x').push(true);

    let (f, b) = q.front_back_mut();
    *f += 1;
    let (f, b) = b.front_back_mut();
    *f = 'y';
    *b.front_mut() = false;

    assert_eq!(q.as_tuple(), (&43, &'y', &false));
}
