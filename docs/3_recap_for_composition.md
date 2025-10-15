# Recap for Composition

So far, with the queue types we have achieved:

* to define a statically-typed collection of heterogeneous types, or equivalently, to define ad-hoc structs,
* to create a generic type-safe builder that lets us build any queue or any struct.

However, we are still far away from the main goal.

The main goal, which is not discussed thus far, is to have a heterogeneous collection of elements with a shared behavior. This collection must have a dynamic feel for ergonomics but must be statically typed for performance.

If we go with the classical example, which is also used in the rust-book's [trait objects chapter](https://doc.rust-lang.org/book/ch18-02-trait-objects.html), we want a heterogeneous collection of things that we can draw, or that implements a trait `Draw`.

The example goes as follows:

* We have a `Draw` trait defined by the `fn draw(&self)` method.
* A bunch of types implement this trait such as `Button` or `SelectBox`.
* Finally, we have a `Screen` which holds a heterogeneous collection of components implementing `Draw`. This allows the screen to draw all components.

We will approach to this problem from a very different perspective than the one in the book.

## Defining Screen via Composition

Let's define our `Screen` as a queue of heterogeneous types all of which implement `Draw`.

The key idea of this approach is related to the question:

> *what if we also require queue implementations to implement `Draw`?*

In other words, we require `QueueMeta: Draw`. Recall that we have two concrete queue implementations: `EmptyQueue` and non-empty `Queue`.

### Draw on an empty queue

What should `EmptyQueue::draw(&self)` do?

* This is where we define the **identity**.
* The sensible thing to do for the draw example seems to be nothing.

```rust
impl Draw for EmptyQueue {
    fn draw(&self) {}
}
```

### Draw a non-empty queue

Recall that a non-empty `Queue` is composed of two parts, the **front** element, and the **back** defining the queue of remaining elements.

As we mentioned in the beginning, all elements in the queue implement `Draw`, and so does the **front**.

Further, since the **back** itself is a queue and we additionally require every queue implementation to implement `Draw`, **back** also has the draw implementation.

Back to the question.

What should `Queue:draw(&self)` do?

* This is where we define **composition**.
* A queue is composed of a front element that is `Draw` and a back queue that is also `Draw`. The right thing to do is probably to draw them both. You may decide on the order if it matters.

```rust
impl<F, B> Draw for Queue<F, B>
where
    F: Draw,
    B: QueueMeta,
{
    fn draw(&self) {
        self.front.draw();
        self.back.draw();
    }
}
```
