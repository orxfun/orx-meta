//! There exist 2 queue traits and 3 implementations to represent a strongly-typed queue of anything:
//! * `Queue`: is the general queue trait, all of `EmptyQueue`, `Single` and `Queue` implement this trait;
//! * `NonEmptyQueue`: this trait excludes the empty queue; hence, implemented by `Single` and `Queue`.
//!
//! This example demonstrates how to use the Queue as a collection of anything.
//!
//! When we say anything, it can literally be anything, like `Any`.
//! And, as we know, `Any` does not have any capabilities ;)
//! Therefore, we can only push & pop them in and out of the queue.
//!
//! Although the usage looks dynamic, the queue is strongly-typed.

use orx_meta::queue::{EmptyQueue, Queue, QueueMeta};
use orx_meta::queue_of;

fn main() {
    // # 1. build queue by pushing elements of arbitrary type

    // empty queue
    let queue = EmptyQueue::new();
    assert!(queue.is_empty());

    // queue with a single element of i32
    let queue = EmptyQueue::new().push(42); // or
    let queue = Queue::new(42);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.front(), &42);

    // queue with two elements: i32 and bool
    let queue = EmptyQueue::new().push(42).push(true);
    assert_eq!(queue.as_tuple(), (&42, &true));

    // queue with four elements: i32, bool, char and &str
    let queue = EmptyQueue::new().push(42).push(true).push('x').push("foo");
    assert_eq!(queue.as_tuple(), (&42, &true, &'x', &"foo"));

    // # 2. underlying queue type

    // notice that the queue is statically typed
    let queue: Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>> = queue;

    // might be easier to use queue_of macro to define the type though
    let queue: queue_of!(i32, bool, char, &'static str) = queue;

    // # 3. access elements recursively

    assert_eq!(queue.front(), &42);

    let back: &queue_of!(bool, char, &'static str) = queue.back();
    assert_eq!(back.front(), &true);

    let back: &queue_of!(char, &'static str) = back.back();
    assert_eq!(back.front(), &'x');

    let back: &queue_of!(&'static str) = back.back();
    assert_eq!(back.front(), &"foo");

    let back: &queue_of!() = back.back();
    assert!(back.is_empty());

    assert_eq!(queue.back().back().back().front(), &"foo");

    // mutate elements
    let mut queue = queue;

    *queue.front_mut() += 1;
    assert_eq!(queue.front(), &43);

    *queue.back_mut().front_mut() = false;
    assert_eq!(queue.back().front(), &false);

    // # 4. access elements via tuple representation

    let mut queue = EmptyQueue::new().push(42).push(true).push('x').push("foo");
    let (a, b, c, d) = queue.as_tuple();
    assert_eq!(a, &42);
    assert_eq!(b, &true);
    assert_eq!(c, &'x');
    assert_eq!(d, &"foo");

    let (a, b, c, d) = queue.as_tuple_mut();
    *a += 1;
    *b = false;
    *c = 'y';
    *d = "bar";
    assert_eq!(queue.as_tuple(), (&43, &false, &'y', &"bar"));

    let (a, b, c, d) = queue.into_tuple();

    assert_eq!((a, b, c, d), (43, false, 'y', "bar"));

    // # 5. pop elements from the queue

    let queue = EmptyQueue::new().push(42).push(true).push('x').push("foo");

    let (num, queue) = queue.pop();
    assert_eq!(num, 42);
    assert_eq!(queue.as_tuple(), (&true, &'x', &"foo"));

    let (flag, queue) = queue.pop();
    assert_eq!(flag, true);
    assert_eq!(queue.as_tuple(), (&'x', &"foo"));

    let (char, queue) = queue.pop();
    assert_eq!(char, 'x');
    assert_eq!(queue.as_tuple(), (&"foo"));

    let (name, queue) = queue.pop();
    assert_eq!(name, "foo");
    assert!(queue.is_empty());
}
