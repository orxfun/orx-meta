# Solution with Macros

> *you may find the example code [here](https://github.com/orxfun/orx-meta/blob/main/examples/5_solution_with_macros.rs)*

The following are our notes on the problem from the previous chapter:

* It is as simple as string replacement.
* Unfortunately, we are not able to represent this with the type system.

When we hear these two, macros come to the rescue.

**orx_meta** crate defines the non-restricted queue types `StQueue`, `EmptyQueue` and `Queue` for demonstration and for general use cases.

But it also provides the `define_queue` macro to let you define your queues with heterogeneous elements all of which share a custom behavior. It handles straightforward boilerplate and leaves the critical implementations of the custom behavior for empty (identity) and non-empty (composition) queues to you.

The entire queue definition described in chapter 3 can be created as follows:

```rust
orx_meta::define_queue!(
    elements => [ Draw ];
    queue => [ StQueue ; EmptyQueue, Queue ];
);

impl Draw for EmptyQueue {
    fn draw(&self) {}
}

impl<F, B> Draw for Queue<F, B>
where
    F: Draw,
    B: StQueue,
{
    fn draw(&self) {
        self.f.draw();
        self.b.draw();
    }
}
```

In the `queue =>` block, we provide three names of the types:
* the queue trait `StQueue`,
* and two implementations: the empty queue struct `EmptyQueue` and non-empty queue struct `Queue`.

To be consistent, we provided the same names, but the names can be anything.

In the `elements =>` block, we provide a comma-separated list of traits that we want to restrict our elements with. In this example, we require all elements of the queue to implement `Draw`.

Finally, we must implement all of the traits listed in the `elements` block for the empty and non-empty queues to define the behavior.

This is sufficient to have our screen to run:

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
screen.run();

// prints out:

// Button { width: 3, height: 4, label: "login" }
// Button { width: 4, height: 5, label: "logout" }
// SelectBox { width: 10, height: 6, options: ["This", "that"] }
```
