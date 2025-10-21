use crate::{
    nonempty_queue::{Queue, QueueBuilder, QueueSingle},
    test_nonempty_queue_of,
};

#[test]
fn nonempty_queue_builder() {
    type Q1 = test_nonempty_queue_of!(u32);
    let q = QueueBuilder::<Q1>::new().push(42).finish();
    assert_eq!(q.into_tuple(), 42);

    type Q2 = test_nonempty_queue_of!(u32, char);
    let q = QueueBuilder::<Q2>::new().push(42).push('x').finish();
    assert_eq!(q.into_tuple(), (42, 'x'));

    type Q3 = test_nonempty_queue_of!(u32, char, bool);
    let q = QueueBuilder::<Q3>::new()
        .push(42)
        .push('x')
        .push(true)
        .finish();
    assert_eq!(q.into_tuple(), (42, 'x', true));

    type Q4 = test_nonempty_queue_of!(u32, char, bool, String);
    let q = QueueBuilder::<Q4>::new()
        .push(42)
        .push('x')
        .push(true)
        .push("foo".to_string())
        .finish();
    assert_eq!(q.into_tuple(), (42, 'x', true, "foo".to_string()));
}
