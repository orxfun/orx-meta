//! We can create a plain queue with `define_queue` macro.
//! This will create two queue traits and three implementations:
//! * `Queue`: is the general queue trait, all of `Empty`, `Single` and `Pair` implement this trait;
//! * `NonEmptyQueue`: this trait excludes the empty queue; hence, implemented by `Single` and `Pair`.

use orx_meta::define_queue;

// define a plain queue
define_queue!(
    queue => [ Queue, NonEmptyQueue ; Empty, Single, Pair ];
);

fn main() {
    // initiate an empty queue
    let queue = Empty::new();
    assert!(queue.is_empty());

    // push elements to the back
    let queue = queue.push(42);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front(), &42);

    let queue = queue
        .push('x')
        .push(true)
        .push(String::from("xyz"));
    assert_eq!(queue.len(), 4);
    assert_eq!(queue.front(), &42);

    // pop elements from the front
    let (popped, queue) = queue.pop();
    assert_eq!(popped, 42);

    let (popped, queue) = queue.pop();
    assert_eq!(popped, 'x');

    let (popped, queue) = queue.pop();
    assert_eq!(popped, true);

    let (popped, queue) = queue.pop();
    assert_eq!(popped, String::from("xyz"));

    assert!(queue.is_empty());

    // as & into tuple
    let queue = Empty::new()
        .push(42)
        .push('x')
        .push(true)
        .push(String::from("xyz"));

    let (a, b, c, d) = queue.as_tuple();
    assert_eq!(a, &42);
    assert_eq!(b, &'x');
    assert_eq!(c, &true);
    assert_eq!(d, &String::from("xyz"));

    let (a, b, c, d) = queue.into_tuple();
    assert_eq!(a, 42);
    assert_eq!(b, 'x');
    assert_eq!(c, true);
    assert_eq!(d, String::from("xyz"));

    // from tuple

    let queue = Pair::from((a, b, c, d));
    assert_eq!(queue.as_tuple(), (&42, &'x', &true, &String::from("xyz")));
}
