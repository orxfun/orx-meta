# Recap for Composition

> *you may find the example code [here](https://github.com/orxfun/orx-meta/blob/main/examples/3_composition_ideation.rs)*

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

But there is a problem. You might've noticed that we do not import the queues from the **orx_meta** crate in the [example file]((https://github.com/orxfun/orx-meta/blob/main/examples/3_composition_ideation.rs)). Instead, we re-implemented them. The problem is described in the next section, and then comes the solution.
