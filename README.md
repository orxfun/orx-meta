# orx-meta-queue

[![orx-meta crate](https://img.shields.io/crates/v/orx-meta.svg)](https://crates.io/crates/orx-meta)
[![orx-meta crate](https://img.shields.io/crates/d/orx-meta.svg)](https://crates.io/crates/orx-meta)
[![orx-meta documentation](https://docs.rs/orx-meta/badge.svg)](https://docs.rs/orx-meta)

Meta structures such as statically typed queues of heterogeneous elements.

## A. queue module

For detailed documentation, please see the sections:
1. [Collection of Anything](https://github.com/orxfun/orx-meta/blob/main/docs/1_collection_of_anything.md)
2. [Generic Builder](https://github.com/orxfun/orx-meta/blob/main/docs/2_generic_builder.md)
3. [Composition Idea](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)
4. [Problem with Type System](https://github.com/orxfun/orx-meta/blob/main/docs/4_problem_with_type_system.md)
5. [Solution with Macros](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)
6. [Summary](https://github.com/orxfun/orx-meta/blob/main/docs/6_summary.md)

The queue module defines a statically typed queue of heterogeneous elements. Further, it provides a macro to define these queue types which are bounded by a specific set of traits representing the common behavior of elements.

These definitions are a bit confusing, it is better to see what we can achieve with some examples.

### A. Zero-Cost Composition

Please see the [zero-cost composition article](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15) for details.

Consider the classical problem about polymorphism, which is also used in rust book's [trait objects chapter](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).

* We have a `Draw` trait and various components, such as button and select box, implement this trait.
* The `Screen` is a collection of components that we can draw.
* Three methods are required for the screen.
* Screen is sort of a collection, so we need `new` to create an empty screen and `push` to add components.
* The third method `draw` is related to the common behavior and draws all components on the screen.

Two well-known and different solutions are the trait object & enum solutions.

Composition idea with statically-typed queues leads to a third approach which is considerably different.

We first set up the draw trait and a couple of implementations for demonstration. Then, we provide the solution.

```rust
// # SET UP

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    fn new(width: u32, height: u32, label: String) -> Self {
        Self { width, height, label }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("{self:?}");
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self { width, height, options }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}

// # SOLUTION

orx_meta::define_queue!(
    elements => [ Draw ];
    queue => [ StScreen; EmptyScreen, Screen ];
);

impl Draw for EmptyScreen {
    // identity: do nothing
    fn draw(&self) {}
}

impl<F: Draw, B: StScreen> Draw for Screen<F, B> {
    // composition: draw them both
    fn draw(&self) {
        self.f.draw();
        self.b.draw();
    }
}

let screen = EmptyScreen::new()
    .push(Button::new(3, 4, "home".to_string()))
    .push(Button::new(5, 4, "about".to_string()))
    .push(SelectBox::new(5, 4, vec!["one".to_string()]))
    .push(Button::new(6, 6, "login".to_string()));
screen.draw();
```

The solution looks concise and ergonomic, similar to the alternative approaches with trait objects or enums.

This implementation provides us the following properties.

#### **<span style="color:green">pros</span>**

* It is open for extension. Another codebase can define a new component, implement `Draw` for it and add it to the screen.
* No heap allocation required for the components. There is not even an allocation for a `Vec`.
* No virtual method calls, all draw calls are statically dispatched. Further, there is no run-time branching. screen.draw() call can completely be inlined by the compiler as `btn1.draw(); btn2.draw(); sbox.draw(); btn3.draw();`.
* No boilerplate code is required. We only define the **identity** and **composition** definitions of the shared behavior, `Draw`.

#### **<span style="color:red">cons</span>**

* `Screen` is a new type specific to the `Draw` trait, it has two generic parameters and it is more complex than the `Vec` wrappers that can be used in the alternative solutions.

#### The Macro

[`define_queue`](https://docs.rs/orx-meta/latest/orx_meta/macro.define_queue.html) macro contains two blocks:

* The `queue` block is just giving names to (i) the statically-typed queue trait, (ii) empty queue struct and (iii) non-empty queue struct.
* The `elements` block is the important part. Here, we provide a comma-separated list of traits that define the common behavior of heterogeneous elements of the queue.

Then, the macro defines the queue types exactly as we saw in the expansion with only one difference. It adds the traits listed in elements as requirements to elements of the queues, and it requires the queues to implement these traits themselves.

* We implement `Draw for EmptyScreen` where we define the behavior in the absence of an element.
* We implement `Draw for Screen` where we define how to compose the common behavior when there are multiple elements.

Note that it is possible to omit the `elements` block in the macro. In this case, we could've added literally any elements to the queue, while the queue would be statically-typed in its elements.

### B. Generic Builder

And a statically-typed queue of anything is an ad-hoc struct.

We already have tuples for this.

However, incremental build capability of queues come in handy. For instance, it allows us to create a generic builder that we can use for any struct. Since the queues are statically-typed in its elements, the builder prevents calling push with wrong types or in wrong order, and prevents us from finishing early or late.

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
    .push(42) // cannot call with wrong type, or
    .push(true) // cannot call in wrong order
    .push('x')
    .push("foo".to_string())
    .finish() // cannot finish before pushing all fields
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

Note that any `struct` can be represented as a queue, type of which can be obtained by the [`queue_of`](https://docs.rs/orx-meta/latest/orx_meta/macro.queue_of.html) helper macro.

If we want to use the queue builder, it is helpful and straightforward to provide `From` queue implementation for the complex struct.

Then, we can create a [`QueueBuilder`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.QueueBuilder.html) with this particular queue type as its generic argument, defining the target type to achieve. And we can safely build our complex type.

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-meta/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
