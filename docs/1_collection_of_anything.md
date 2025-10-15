# Collection of Anything

> *you may find the example code [here](https://github.com/orxfun/orx-meta/blob/main/examples/1_collection_of_anything.rs)*

How do we have a collection of arbitrary types in rust?

> *And why would we ever want that? We park this question for a while. Hopefully, things will get more useful as we proceed.*

Can we have a collection of anything?

## Dynamic collection of `Any`thing

Yes, we can actually have any collection of elements of arbitrary types by defining our elements as trait objects of the [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait.

```rust
use std::any::Any;

let mut any_vec: Vec<Box<dyn Any>> = vec![];
any_vec.push(Box::new(42));
any_vec.push(Box::new(true));
any_vec.push(Box::new('x'));
any_vec.push(Box::new("foo"));
```

Our vector contains four elements of types `i32`, `bool`, `char` and `&str`.

We cannot do much with elements of `Vec<Box<dyn Any>>` because `Any` does not have any abilities :)

In the following, `element` can be anything. All we know is its type id. This might allow us to make it useful, but only with advanced dynamic techniques.

```rust
let element = any_vec.pop().unwrap();
println!("{:?}", element.type_id());
```

## Statically-typed collection of anything

The `queue` module of the [orx_meta](https://github.com/orxfun/orx-meta/) crate defines three types:

* [`StQueue`](https://docs.rs/orx-meta/latest/orx_meta/queue/trait.StQueue.html) trait defines meta information and push operation of queues.
* [`EmptyQueue`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.EmptyQueue.html) is the first queue implementation and its self-explanatory.
* [`Queue`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.Queue.html), on the other hand, is a non-empty queue.

Let's see how we could define a collection of four elements of types `i32`, `bool`, `char` and `&str` with these types.

```rust
use orx_meta::queue::*;

let queue = EmptyQueue::new().push(42).push(true).push('x').push("foo");
// or
let queue = Queue::new(42).push(true).push('x').push("foo");
```

Now, this is quite different from the `any_vec`.

It is statically typed in the sense that you may observe the types of its elements from its signature. The type of the `queue` is `Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>>`.

You may keep pushing elements to the queue. Unlike pushing to a vec that requires `&mut self`, pushing to the queue require `self`. This is nice, the function is pure and the signature allows chaining push calls. But also this is the only way because every time we push to the queue, its type changes. The following break down of the calls reveals the changes in the type.

```rust
use orx_meta::queue::*;

let queue: EmptyQueue = EmptyQueue::new();
let queue: Queue<i32, EmptyQueue> = queue.push(42);
let queue: Queue<i32, Queue<bool, EmptyQueue>> = queue.push(true);
let queue: Queue<i32, Queue<bool, Queue<char, EmptyQueue>>> = queue.push('x');
let queue: Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>> =
    queue.push("foo");
```

> Notice the recursive type definition where empty queue is used as the stopping condition.

Since we know the types of its elements, we can use them with their natural properties. First, we need to represent the queue with two components:
* `front`: this is the element in the front of the queue, the item to be popped.
* `back`: this is the queue containing all elements except for the one in the front.

You may then access the third element of the queue with `queue.back().back().front()`.

```rust
use orx_meta::queue::*;

let mut queue = Queue::new(42).push(true).push('x').push("foo");

let num = queue.front() * 2; // i32
assert_eq!(num, 84);

*queue.back_mut().front_mut() = false; // bool

assert_eq!(queue.back().back().front(), &'x'); // char

*queue.back_mut().back_mut().back_mut().front_mut() = "bar"; // &str
```

Since it is a queue, we can pop elements from the front, which breaks the queue into two pieces: (i) its front or the popped element, and (ii) its back or the remaining queue.

```rust
use orx_meta::queue::*;

let queue = Queue::new(42).push(true).push('x').push("foo");

let (num, queue) = queue.pop();
assert_eq!(num, 42);

let (flag, queue) = queue.pop();
assert_eq!(flag, true);

let (ch, queue) = queue.pop();
assert_eq!(ch, 'x');

let (name, queue) = queue.pop();
assert_eq!(name, "foo");

// queue.pop(); // doesn't compile; pop does not exist for EmptyQueue!
assert!(queue.is_empty());
```

# Interpretation as an ad-hoc struct

To recap the queue:
* it can hold elements of arbitrary types,
* its type signature defines the types of all of its elements.

This is nothing but a `struct`.

Under the hood, `MyStruct` and `MyQueue` are equivalent. And `push` calls are equivalent to setting the fields of a struct.

```rust
use orx_meta::queue::*;

struct MyStruct(i32, bool, char, &'static str);
let my_struct = MyStruct(42, true, 'x', "foo");

type MyQueue = Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>>;
let my_queue = Queue::new(42).push(true).push('x').push("foo");
```

The difference is that `MyStruct` is a named type while `MyQueue` is just a type alias for queue. `EmptyQueue` and `Queue` together can represent all possible structs. Therefore, it can be considered as an ad-hoc struct.

> *just like tuples*

Now consider, a type `A` with three fields `i32, bool, char`; and another type `B` including `A` and an additional field `&str`. Queues are useful for representing such incremental differences, converting one type to the other.

```rust
use orx_meta::queue::*;

type A = Queue<i32, Queue<bool, Queue<char, EmptyQueue>>>;
type B = <A as StQueue>::PushBack<&'static str>;

let a: A = Queue::new(42).push(true).push('x');
let b: B = Queue::new(42).push(true).push('x').push("foo");
let b: B = a.push("foo"); // B from A
```

So far, this has been fun with types!

You might be thinking:

* we could've represented ad-hoc structs as a stack, rather than a queue so that we could go to `A from B`,
* or even better, as a double-ended queue so that we could go in both directions,
* or we could've just worked on extending capabilities of tuples with macros, which are already meant to represent ad-hoc structs.

However, the main goal here is establishing an approach to define adhoc composition, rather than re-defining ad-hoc structs. The queue is a choice to achieve this conveniently.

For the latter, please also see the super useful `HList` macro of the [frunk](https://crates.io/crates/frunk#hlist) crate.
