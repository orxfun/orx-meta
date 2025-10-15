# Generic Builder

> *you may find the example code [here](https://github.com/orxfun/orx-meta/blob/main/examples/2_generic_builder.rs)*

## Type Alias Helper

Before moving on, we will introduce a helper macro to define queue types.

Consider a queue of four elements with signature `Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>>`.

The corresponding tuple would be `(i32, bool, char, &'static str)`.

Note the difference. The queue definition is recursive. This allows it to grow arbitrarily. On the other hand, the type signature gets complicated as the queue gets larger. Flat tuple type is much easier to hand-write.

Therefore, the [`queue_of`](https://docs.rs/orx-meta/latest/orx_meta/macro.queue_of.html) type helper macro is defined.

```rust
use orx_meta::queue::*;
use orx_meta::queue_of;

type Q = Queue<i32, Queue<bool, Queue<char, Queue<String, EmptyQueue>>>>;
let q: Q = Queue::new(42).push(true).push('x').push("foo".to_string());

type R = queue_of!(i32, bool, char, String); // R == Q
let r: R = q;
```

## Type-safe Generic Builder for any Queue

Assume we require an instance of a particular queue type. Since the queues are statically typed, we cannot create and use a queue with elements of the wrong types.

On the other hand, the `push` method on the queue accepts any type of element. Since we have a target queue type to achieve, it would be helpful if `push` accepted only the correct type. And it would be helpful if we could not stop halfway or if we cannot go beyond the target.

All these describe the builder pattern.

The [`QueueBuilder`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.queue_of.html) is a generic builder for any type of queue providing the abovementioned properties.

The following example demonstrates how to create an instance of `queue_of!(u32, bool, char, String)` with the builder, making sure that we cannot go wrong.

```rust
use orx_meta::queue::*;
use orx_meta::queue_of;

// this is our target type that we need instances of
type MyQueue = queue_of!(u32, bool, char, String);

// notice that `push` can only be called with the correct type
// and finish is available only when all elements are pushed
let val: MyQueue = QueueBuilder::<MyQueue>::new()
    .push(42)
    .push(true)
    .push('x')
    .push("foo".to_string())
    .finish();1
assert_eq!(val.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
```

## Type-safe Generic Builder for any Complex Struct

Recall that any struct can be represented as a queue. Then, we can use the queue builder as a generic builder for any struct.

In the following example, assume we want to have a builder for our `ComplexStruct`. We can conveniently use the generic `QueueBuilder` by implementing `From<queue_of!(u32, bool, char, String)>` for our type.

```rust
use orx_meta::queue::*;
use orx_meta::queue_of;

#[derive(PartialEq, Eq, Debug)]
struct ComplexStruct {
    a: u32,
    b: bool,
    c: char,
    d: String,
}

impl From<queue_of!(u32, bool, char, String)> for ComplexStruct {
    fn from(queue: queue_of!(u32, bool, char, String)) -> Self {
        let (a, b, c, d) = queue.into_tuple();
        Self { a, b, c, d }
    }
}

let val: ComplexStruct = QueueBuilder::<queue_of!(u32, bool, char, String)>::new()
    .push(42)
    .push(true)
    .push('x')
    .push("foo".to_string())
    .finish()
    .into();

assert_eq!(
    val,
    ComplexStruct {
        a: 42,
        b: true,
        c: 'x',
        d: "foo".to_string()
    }
);
```
