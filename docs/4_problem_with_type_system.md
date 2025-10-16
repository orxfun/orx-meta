# Problem with the Type System

I am a big fan of the rust type system ❤️ that's my favorite feature of rust.

However, I have not been able to express the queue of heterogeneous elements all of which implement a particular trait within the type system.

## Current: Queue of Heterogeneous Elements

The following is a little shortened version of queue types defined in the **orx_meta::queue** module.

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

// empty queue
struct EmptyQueue;

impl StQueue for EmptyQueue {
    type PushBack<T> = Queue<T, EmptyQueue>; // a queue of 1 element

    type Front = Self;

    type Back = Self;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue { front: element, back: EmptyQueue }
    }
}

// a non-empty queue
struct Queue<F, B>
where
    B: StQueue,
{
    front: F,
    back: B,
}

impl<F, B> StQueue for Queue<F, B>
where
    B: StQueue,
{
    type PushBack<T> = Queue<F, B::PushBack<T>>;

    type Front = F;

    type Back = B;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue {
            front: self.front,
            back: self.back.push(element)
        }
    }
}
```

## Desired: Queue of Heterogeneous Elements with a Shared Behavior

This time, we want all elements to belong to a class of types, rather than anything. Without loss of generalization, let's say we want them all to implement the `CommonBehavior` trait.

We *almost* end up with the same implementations. The differences are highlighted with comments.

```rust
// # 1: we require each element of the queue to implement this
trait CommonBehavior {}

// # 3: we require both of the queue implementations to implement CommonBehavior
trait StQueue: CommonBehavior {
    // # 2: we can only push elements implementing CommonBehavior
    type PushBack<T>: StQueue
    where
        T: CommonBehavior;

    // # 2: we can only have elements implementing CommonBehavior
    type Front: CommonBehavior;

    type Back: StQueue;

    // # 2: we can only push elements implementing CommonBehavior
    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: CommonBehavior;
}

// empty queue
struct EmptyQueue;

impl StQueue for EmptyQueue {
    type PushBack<T>
        = Queue<T, EmptyQueue>
    where
        T: CommonBehavior;

    type Front = Self;

    type Back = Self;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: CommonBehavior,
    {
        Queue {
            front: element,
            back: EmptyQueue,
        }
    }
}

// # 3-a: implement CommonBehavior for empty queue: define identity
impl CommonBehavior for EmptyQueue {}

// a non-empty queue
struct Queue<F, B>
where
    // # 2: we can only have elements implementing CommonBehavior
    F: CommonBehavior,
    B: StQueue,
{
    front: F,
    back: B,
}

impl<F, B> StQueue for Queue<F, B>
where
    F: CommonBehavior,
    B: StQueue,
{
    type PushBack<T>
        = Queue<F, B::PushBack<T>>
    where
        T: CommonBehavior;

    type Front = F;

    type Back = B;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: CommonBehavior,
    {
        Queue {
            front: self.front,
            back: self.back.push(element),
        }
    }
}

// # 3-b: implement CommonBehavior for non-empty queue: define composition
impl<F, B> CommonBehavior for Queue<F, B>
where
    F: CommonBehavior,
    B: StQueue,
{
}
```

- **# 1: we require each element of the queue to implement this**
  * This is the shared behavior, similar to our `Draw` trait in the screen example.
  * We want heterogeneous types with the restriction that all of them must implement `CommonBehavior`.
- **# 2: we can only push/have elements implementing CommonBehavior**
  * To make sure that the queue only contains elements implementing `CommonBehavior`, we need to constraint the element type on the `push` method. Similarly, the front of the non-empty queue must implement the common behavior trait.
- **# 3: we require both of the queue implementations to implement CommonBehavior**
  * Following up on the ideation described in the previous section, we require our queue implementations to implement the common behavior as well. This allows us to define composition. We need two implementations.
  * **3-a:** we implement the common behavior for the empty queue, this defines the identity behavior.
  * **3-b:** we also implement it for the non-empty queue to describes how to composed multiple elements for the common behavior.

## The Problem

Notice that there are minor differences in the two implementations, which can be split into two parts.

First group is **# 3**, the implementations of the `CommonBehavior` for the empty and non-empty queues. Note that the common behavior can be anything and the implementations have to be custom anyways. This is where we explicitly define the identity and composition behavior, so it is not boilerplate.

The changes in the second group **# 1** and **# 2**, on the other hand, are pretty generic. We could replace `CommonBehavior` for any trait, say `X`, and we would have exactly the same changes. It is as simple as string replacement.

```rust
trait StQueue: X {
    type PushBack<T>: StQueue
    where
        T: X;

    type Front: X;

    type Back: StQueue;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: X;
}
```

Unfortunately, we are not able to represent this with the type system. What we want and what won't compile is the following:

```rust
trait StQueue<X>: X {
    type PushBack<T>: StQueue<X>
    where
        T: X;

    type Front: X;

    type Back: StQueue<X>;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: X;
}
```

Here, the generic `X` is not a concrete type but it is a **trait**.

We want our `StQueue` trait to be generic over another trait `X`.

Sadly, this is not possible today.

But there is always a solution with rust.

[<|](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)======[|>](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)
