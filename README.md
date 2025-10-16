# orx-meta-queue

[![orx-meta crate](https://img.shields.io/crates/v/orx-meta.svg)](https://crates.io/crates/orx-meta)
[![orx-meta crate](https://img.shields.io/crates/d/orx-meta.svg)](https://crates.io/crates/orx-meta)
[![orx-meta documentation](https://docs.rs/orx-meta/badge.svg)](https://docs.rs/orx-meta)

Meta structures such as statically typed queues of heterogeneous elements.

## queue module

For detailed documentation, please see the sections:
1. [Collection of Anything](https://github.com/orxfun/orx-meta/blob/main/docs/1_collection_of_anything.md)
2. [Generic Builder](https://github.com/orxfun/orx-meta/blob/main/docs/2_generic_builder.md)
3. [Composition Idea](https://github.com/orxfun/orx-meta/blob/main/docs/3_composition_idea.md)
4. [Problem with Type System](https://github.com/orxfun/orx-meta/blob/main/docs/4_problem_with_type_system.md)
5. [Solution with Macros](https://github.com/orxfun/orx-meta/blob/main/docs/5_solution_with_macros.md)
6. [Summary](https://github.com/orxfun/orx-meta/blob/main/docs/6_summary.md)

The queue module defines a statically typed queue of heterogeneous elements. Further, it provides a macro to define these queue types which are bounded by a specific set of traits; i.e., common behavior of the elements.

These definitions are a bit confusing, it is better to see what we can achieve with some examples.

### A. Zero-Cost Composition

Please see the [zero-cost composition article](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15) for details.

Consider the classical problem about polymorphism, which is also used in rust book's [trait objects chapter](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).

* We have a `Draw` trait and various components such as button and select box implement this trait.
* We have a `Screen` which is a collection of components that we can draw.
Three methods are required for the screen.
* Since screen is a collection of components, we need `new` to create an empty screen and `push` to add a component to it.
* The third method `draw` is related to the common behavior and draws all components on the screen.

We can solve this problem using trait objects or using enums.

Using the composition idea described here, we can also solve this with statically-typed queue as follows:

```rust
// set up
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

// solution by zero-cost composition

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

The [`define_queue`](https://docs.rs/orx-meta/latest/orx_meta/macro.define_queue.html) macro contains two blocks:

* The `queue` block is straightforward. We just provides the names of the three types. We name the statically typed queue trait as `StScreen`, the empty queue struct as `EmptyScreen` and the non-empty queue struct as `Screen`.
* The `elements` block is central to the zero-cost composition idea. We provide a comma-separated list of traits that each of the heterogeneous elements of the queue must implement. This prevents us from pushing an element without `Draw` implementation to the screen.

Furthermore, we require the empty queue and non-empty queue structs to implement the common behavior.

* We implement `Draw for EmptyScreen` where we define the behavior in the absence of an element.
* We implement `Draw for Screen` where we define how to compose the common behavior when there are multiple elements.

And this is sufficient to achieve the following **<span style="color:green">pros</span>**:

* It is open for extension. Another codebase can implement a new component and add it to the screen.
* No heap allocation required. Memory layout of the `screen` above is identical to the `struct MyScreen(Button, Button, SelectBox, Button)`. Further, there is not even an allocation for the `Vec`.
* No virtual method calls, all `draw` calls are statically dispatched. Further, there is no run-time branching. Final `screen.draw()` call can completely be inlined by the compiler as `btn1.draw(); btn2.draw(); sbox.draw(); btn3.draw();`.

On the other hand, this approach has the following **<span style="color:red">con</span>** compared to the alternatives:

* `Screen` is a new type specific to the `Draw` trait, has two generic parameters and it is more complex than the `Vec` wrappers used in the alternative approaches.

Notice that the types and macros defined in this crate aim to overcome this complexity and achieve the convenient use pattern illustrated above.

### B. Generic Builder

Note that it is possible to omit the `elements` block in the macro. In this case, we could've added literally any elements to the queue, while the queue would be statically-typed in its elements.

This is an ad-hoc struct.

We already have ad-hoc structs though, tuples.

However, the statically-typed queue can still be useful in other ways, one of which is to define a generic type-safe builder for anything.

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

Note that each `struct` can be represented as a queue, type of which can be obtained by the [`queue_of`](https://docs.rs/orx-meta/latest/orx_meta/macro.queue_of.html) helper macro.

It is straightforward to implement `From` queue for the struct.

Then, we can create a [`QueueBuilder`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.QueueBuilder.html) with this particular queue type as its generic argument, defining the target type to achieve. And we can safely build our complex type.

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-meta/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
