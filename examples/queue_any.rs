use orx_meta::{define_queue_core_zzz, define_queue_tuple_transformation_zzz};

define_queue_core_zzz!(
    names => {
        traits: { queue: Queue, non_empty_queue: NonEmptyQueue },
        structs: { empty: Empty, single: Single, pair: Pair }
    };
);

fn as_queue_of_different_type_elements() {
    let q = Empty::new().push('x').push(12).push(true);

    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);

    let (f, q) = q.pop();
    assert_eq!(f, 'x');
    assert_eq!(q.front(), &12);
    assert_eq!(q.len(), 2);

    let (f, q) = q.pop();
    assert_eq!(f, 12);
    assert_eq!(q.front(), &true);
    assert_eq!(q.len(), 1);

    let (f, q) = q.pop();
    assert_eq!(f, true);
    assert!(q.is_empty());
}

define_queue_tuple_transformation_zzz!(
    queues => { trait: Queue, empty: Empty, single: Single, pair: Pair };
);

fn basic_builder_pattern() {
    let mut q = Empty::new().push('x').push(12).push(true);

    // into tuple
    let tuple = q.clone().into_tuple();
    assert_eq!(tuple, ('x', 12, true));

    // from tuple
    q = ('y', 42, false).into();
    assert_eq!(
        q,
        Empty::new().push('y').push(42).push(false)
    );
}

fn main() {
    as_queue_of_different_type_elements();
    basic_builder_pattern();
}
