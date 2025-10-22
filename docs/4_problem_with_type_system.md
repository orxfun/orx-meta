# Problem with the Type System

I am a big fan of the rust type system ❤️ that's my favorite feature of rust.

However, I have not been able to express the queue of heterogeneous elements all of which implement a particular trait within the type system.

## Current: Queue of Heterogeneous Elements

The following is a little shortened version of statically-typed queue trait as defined in the **orx_meta::queue** module.

```rust
// queue of statically-typed heterogeneous elements
trait StQueue {
    // type of the queue we get when we push an element of T
    type PushBack<T>: StQueue;

    // type of the front element
    type Front;

    // type of the queue of elements excluding the front
    type Back: StQueue;

    // returns the queue obtained when pushing element to this queue
    fn push<T>(self, element: T) -> Self::PushBack<T>;
}
```

## Desired: Queue of Heterogeneous Elements with a Shared Behavior

This time, we want all elements to belong to a class of types, rather than anything. Without loss of generalization, let's say we want them all to implement the `X` trait.

We *almost* end up with the same implementations. The comments describe the differences.

```rust
// # 1: we require each element of the queue to implement this
trait X {}

// # 2: we require all (both) of the queue implementations to implement X
trait StQueue: X {
    type PushBack<T>: StQueue
    where
        T: X; // # 1

   
    type Front: X; // # 1

    type Back: StQueue;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: X; // # 1
}
```

- **# 1: we require each element of the queue to implement this**
  * This is the shared behavior, similar to our `Draw` trait in the screen example.
  * We want heterogeneous types with the restriction that all of them must implement `X`.
  * To make sure that the queue only contains elements implementing `X`, we need to constraint the element type on the `push` method. Similarly, the front of the queue must implement `X`.
- **# 2: we require both of the queue implementations to implement X**
  * Following up on our zero-cost composition idea, we require our queue implementations to implement the common behavior as well. This allows us to define composition. We need two implementations.
  * **2-a:** we implement the common behavior for queue with single element defining the **identity** behavior.
  * **2-b:** we also implement it for multiple-element queues to define **composition**.

## The Problem

Notice that there are minor differences in the two implementations, which can be split into two parts.

First group is **# 2**, the implementations of the common behavior for queues. This is where we explicitly define the identity and composition behavior, so it is not boilerplate.

The changes in **# 1**, on the other hand, are just adding some trait bounds. If we replaced `X` with `Draw` we would get the queue that we used for our screen example.

Notice that the generic `X` is not a concrete type but it is a **trait**.

We want our `StQueue` trait to be generic over another trait `X`.

Sadly, this is not possible today.

But there is always a solution with rust.

[<|](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)======[|>](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)
