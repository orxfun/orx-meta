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

The queue module defines a statically typed queue of heterogeneous elements. Further, it provides a macro to define these queue types which are bounded by a specific set of traits representing the **common behavior of elements**.

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
    queue => [ StScreen; ScreenSingle, Screen ];
);

impl<F: Draw> Draw for ScreenSingle<F> {
    // identity: just draw the single element
    fn draw(&self) {
        self.f.draw();
    }
}

impl<F: Draw, B: StScreen> Draw for Screen<F, B> {
    // composition: draw them both
    fn draw(&self) {
        self.f.draw();
        self.b.draw();
    }
}

let screen = Screen::new(Button::new(3, 4, "home".to_string()))
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

* `Screen` with generic parameters is a more complicated type than the `Vec` wrappers used in alternative approaches. On the other hand, we can conveniently work with it without needing its concrete type since it can be used as `impl Draw`.
* More importantly, it is statically-typed. This means that concrete types of all of its elements must be known at the compile time.

#### The Macro

[`define_queue`](https://docs.rs/orx-meta/latest/orx_meta/macro.define_queue.html) macro contains two blocks:

* The `queue` block is just giving names to (i) the statically-typed queue trait, (ii) empty queue struct and (iii) non-empty queue struct.
* The `elements` block is the important part. Here, we provide a comma-separated list of traits that define the common behavior of heterogeneous elements of the queue.

Then, the macro defines the queue types exactly as we saw in the expansion with only one difference. It adds the traits listed in elements as requirements to elements of the queues, and it requires the queues to implement these traits themselves.

* We provide the straightforward **identity** definition with `Draw for ScreenSingle` implementation.
* We implement `Draw for Screen` where we define how to **compose** the common behavior when there are multiple elements.

#### Potential Use Case #1: As a Collection

Although it is statically-typed, we can still use the queue as a collection of heterogeneous types when we know types of all elements during compile time. This fits to situations where we instantiate the collection during the start up of a service.

We often parse configuration files into the collection. For instance, we can have a json file listing components to be included in the screen.

In this case, instead of the following initialization using trait objects:

```rust ignore
// # json file
[
    {
        "type": "Button",
        "width": 3,
        "height": 4,
        "label": "home"
    },
    {
        "type": "Button",
        "width": 5,
        "height": 4,
        "label": "about"
    },
    {
        "type": "SelectBox",
        "width": 5,
        "height": 4,
        "options": [
            "one"
        ]
    },
    {
        "type": "Button",
        "width": 6,
        "height": 6,
        "label": "login"
    }
]
    
// # start up code (trait objects)

fn new_screen() -> Vec<Box<dyn Draw>> {
    let data = std::fs::read_to_string("path").unwrap();
    serde_json::from_str(&data).unwrap()
}
```

we can use the following initialization as a statically-typed queue:

```rust ignore
// # start up code (statically-typed queue)

fn new_screen() -> impl Draw {
    Screen::new(Button {
        width: 3,
        height: 4,
        label: "home".to_string(),
    })
    .push(Button {
        width: 5,
        height: 4,
        label: "about".to_string(),
    })
    .push(SelectBox {
        width: 5,
        height: 4,
        options: vec!["one".to_string()],
    })
    .push(Button {
        width: 6,
        height: 6,
        label: "login".to_string(),
    })
}
```

With the queue approach, we need to re-compile the code every time the configuration changes. And due to compiler limitations, the queue cannot have too many elements. As far as I experienced, the compiler seems to struggle when we add more than 256 elements.

On the other hand, initialization can never fail, runtime errors are not possible. Further, the queue brings performance and memory advantages.

For a screen containing 200 components with toy draw implementations, statically-typed queue approach is two times faster than trait objects or enum-components approaches. You may see the corresponding benchmarks [here](https://github.com/orxfun/orx-meta/tree/main/benches).

Further, being memory efficient and not requiring heap allocation makes it beneficial for embedded applications.

You may see the entire example [here](https://github.com/orxfun/orx-meta/tree/main/examples/screen).


#### Potential Use Case #2: Tool to deal with Custom Requirements

This has been the main idea to develop the queues for.

Assume that we are maintaining a performance-critical tool with with, say, 5 features. We want our tool to be useful for as many use cases as possible. Every use case requires a different subset of our features. There exist `2^5-1 = 31` potential use cases. But can we cover them all?

We will not have fun if we need to manually combine the features; or if we lose performance due to abstraction.

On the other hand, if we can define the composition of these features using a statically-typed queue:

* We can work on and maintain each feature in isolation.
* Implementing the 6-th feature allows us cover 32 more potential use cases, making it well-worth the effort.
* And we will have access to all combinations of these features without any manual work and without loss of performance.

Assume, for instance, we have different `Criterion` implementations and we define our `Criteria` as a statically-typed queue. Then, we can conveniently create different subsets of criteria to handle different use cases without loss of performance.

```rust ignore
fn use_case1() {
    println!("# Use case with criteria [Distance]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case2() {
    println!("# Use case with criteria [Distance, Precedence]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new()).push(Precedence::new());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case3() {
    println!("# Use case with criteria [Distance, Capacity]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new()).push(Capacity::new());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}

fn use_case4() {
    println!("# Use case with criteria [Distance, Capacity, Precedence]");
    let tour = Tour(vec![City(0), City(1), City(2), City(3)]);

    let criteria = Criteria::new(Distance::new())
        .push(Capacity::new())
        .push(Precedence::new());
    let status = criteria.evaluate(&tour, Status::default());
    println!("{status:?}");
}
```

You may see the entire example [here](https://github.com/orxfun/orx-meta/tree/main/examples/criteria).




### B. Generic Builder

A statically-typed queue of anything is an ad-hoc struct.

Therefore, incremental build capability of queues is useful for constructing structs. For instance, it allows us to create a generic builder that we can use for any struct. Since the queues are statically-typed in its elements, the builder prevents calling push with wrong types or in wrong order, and prevents us from finishing early or late.

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

If we want to use the queue builder, it is helpful and straightforward to provide `From` queue implementation for the complex struct.

Then, we can create a [`QueueBuilder`](https://docs.rs/orx-meta/latest/orx_meta/queue/struct.QueueBuilder.html) with this particular queue type as its generic argument, defining the target type to achieve. And we can safely build our complex type.

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-meta/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
