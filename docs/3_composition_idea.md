# Recap for Composition

> *you may find the example code [here](https://github.com/orxfun/orx-meta/blob/main/examples/3_composition_idea.rs)*

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

## Defining Identity and Composition for Draw

Let's define our `Screen` as a queue of heterogeneous types all of which implement `Draw`.

The key idea of this approach is related to the question:

> *what if we also require queue implementations to implement `Draw`?*

In other words, we require `StQueue: Draw`. Recall that we have two concrete queue implementations: `EmptyQueue` and non-empty `Queue`.

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

The **back** being a queue might make it a bit confusing in order to how to implement this. An easy way to think about is to consider back as a single `Draw` element; This will be the case when the queue has two elements anyways. And if you properly define the composition for two shapes, it will work for any number of shapes.

```rust
impl<F, B> Draw for Queue<F, B>
where
    F: Draw,
    B: StQueue,
{
    fn draw(&self) {
        self.front.draw();
        self.back.draw();
    }
}
```

### A Note on Performance

Notice that `self.front.draw()` is a direct method call.

How about `self.back.draw()`?

The **back** is a statically typed queue, so we know that this is not a virtual call.

Is it recursion?

Also no. Although, it feels recursive, there is no recursion involved. It can completely be inlined as calls to the concrete front elements.

For instance, assume we have a queue of four elements of types `X1`, `X2`, `X3` and `X4`, all implementing `Draw`. Type of our queue is `Queue<X1, Queue<X2, Queue<X3, Queue<X4, EmptyQueue>>>>`.

Now the `draw` implementation that we defined for the non-empty queue on this queue is identical to the `draw_hand_written` implementation below:

```rust
pub struct X1;
impl Draw for X1 { fn draw(&self) {} }
pub struct X2;
impl Draw for X2 { fn draw(&self) {} }
pub struct X3;
impl Draw for X3 { fn draw(&self) {} }
pub struct X4;
impl Draw for X4 { fn draw(&self) {} }

impl Queue<X1, Queue<X2, Queue<X3, Queue<X4, EmptyQueue>>>> {
    fn draw_hand_written(&self) {
        self.front.draw();                  // X1
        self.back.front.draw();             // X2
        self.back.back.front.draw();        // X3
        self.back.back.back.front.draw();   // X4
        self.back.back.back.back.draw();    // EmptyQueue
    }
}
```

All calls are transparent to the compiler and luckily we do not need to write this:)

## Defining Identity and Composition for Another Example

Notice that composition for `draw` is just sequentially calling them on elements. To demonstrate the flexibility of the approach, it is helpful to look at another [example](https://github.com/orxfun/orx-meta/blob/main/examples/3_composition_idea.rs). Let's say we have the following sum trait:

```rust
pub trait Sum {
    fn sum(self) -> i64;
}
```

Various types of numbers that can be turned into `i64` can implement this, since sum of a single number is itself. Then, we can implement the trait for empty and non-empty queues as follows:

```rust
impl Sum for EmptyQueue {
    fn sum(self) -> i64 {
        0 // identity
    }
}

impl<F: Sum, B: StQueue> Sum for Queue<F, B> {
    fn sum(self) -> i64 {
        self.f.sum() + self.b.sum() // composition: +
    }
}
```

This would allow to write the following code:

```rust
let queue = EmptyQueue::new()
    .push(1i16)
    .push(2i32)
    .push(3i32)
    .push(4i64)
    .push(5i32)
    .push(6i64)
    .push(7i16);
let sum = queue.sum();
assert_eq!(sum, 28);
```

where the `queue.sum()` call can be inlined as `1 + 2 + 3 + 4 + 5 + 6 + 7`.

## Defining Screen as a Statically Typed Queue

It didn't take much to set up the `Draw` implementations of the queues, and now we are ready to define our screen as a statically typed queue.

But first, let's add some example shapes that we can draw.

```rust
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{self:?}");
    }
}

#[derive(Debug)]
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}
```

Finally, our screen implementation with `new`, `push` and `run` methods.

```rust
struct Screen<Q: StQueue>(Q);

impl Screen<EmptyQueue> {
    fn new() -> Self {
        Self(EmptyQueue::new())
    }
}

impl<Q: StQueue> Screen<Q> {
    fn push<S: Draw>(self, component: S) -> Screen<Q::PushBack<S>> {
        Screen(self.0.push(component))
    }

    fn run(&self) {
        self.0.draw();
    }
}
```

This suffices to achieve the following.

```rust
let screen = Screen::new()
    .push(Button {
        width: 3,
        height: 4,
        label: String::from("login"),
    })
    .push(Button {
        width: 4,
        height: 5,
        label: String::from("logout"),
    })
    .push(SelectBox {
        width: 10,
        height: 6,
        options: vec![String::from("This"), String::from("that")],
    });

// draw all components
screen.run();
```

Pretty dynamic look and feel!

But strongly typed.

No box and no virtual function calls, whenever it matters.

> Recall the interpretation of the queue as an ad-hoc struct. The `screen` above is as if we'd hand-written a particular screen struct with two button fields and one select box field.

This is a very strong pattern.

But there is a problem. You might've noticed that we do not import the queues from the **orx_meta** crate in the [example file]((https://github.com/orxfun/orx-meta/blob/main/examples/3_composition_idea.rs)). Instead, we re-implemented them. The problem is described in the next section, and then comes the solution.
