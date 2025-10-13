//! In the previous examples (1_collection_of_anything and 2_builder_for_complex_types), we saw
//! the use cases of a statically-typed queue where elements can be anything.
//!
//! We also mentioned that `Any` does not have any special skills.
//! Therefore, a collection of anything is just a collection.
//!
//! On the other hand, the queue can be a tool to define a collection of various types with a common
//! ability.
//!
//! Note that the most common approach to this problem in rust is to use trait objects and dynamic
//! dispatch. The [trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) chapter
//! in the rust book demonstrates this over a traditional example as follows.
//!
//! We have a `Screen` which is a collection of things that we can `Draw`.
//! `Draw` is a trait defined by the `draw(&self)` method.
//! Different concrete types can implement `Draw` such as a `Button` or a `SelectBox`.
//!
//! To have a useful screen, we require the following methods:
//! * `new` will create a new screen without any collections.
//! * `push(component)` will add the `component` to the components to be drawn on the screen.
//!   Note that whatever the concrete type of `component` is, it implements `Draw`.
//! * `run` will draw all components pushed so far on the screen.
//!
//! You may see below two approaches for this problem:
//! * `dynamic_dispatch` function illustrates the implementation using trait objects and dynamic dispatch.
//!   Note that this is identical to the example provided in the rust-book.
//! * `static_dispatch` function demonstrates how to use the statically typed queue to approach to this
//!   problem. Notice that components of the screen defined by the queue are not boxed, they are inlined
//!   values of each component stored one after the other as if we have a different struct type for each
//!   combination of screens. Further, the `draw` calls are direct rather than vtable calls via dynamic
//!   dispatch.
//!
//! Notice that we cannot use the plain `Queue` of anything that is defined in this crate.
//! To make it useful, we need a `Queue` where each element implements `Draw`.
//! In order to achieve this without all the boiler plate, we use the `define_queue` macro.
//! The following call defines the `Queue` and `NonEmptyQueue` traits and three implementations
//! `Empty`, `Single` and `Multi` with the condition that each element has to implement `Draw`.
//!
//! ```ignore
//! define_queue!(
//!     elements => [Draw];
//!     queue => [Queue, NonEmptyQueue ; Empty, Single, Multi];
//! );
//! ```
//!
//! Further, we need each queue implementation to be `Draw`.
//! This is the most significant difference of this approach:
//! * `Button` and `SelectBox` are the elements and they implement `Draw`.
//! * `Empty` queue must also implement `Draw`. We provide this implementation by explicitly defining
//!   what we should do in the absence of a component. In this example, we do nothing.
//! * `Single<F>` queue must also implement `Draw`. Implementation of a single-element queue is always
//!   trivial. It simply calls `self.f.draw()` where `f` is the contained element.
//! * Finally `Multi<F, B>` queue must implement `Draw`. This implementation is the important bit where we
//!   define how we compose the draw behavior of multiple components. Recall that, `F` here is the element
//!   in the front of the queue and `B` is the queue of the remaining elements. In this example, we simply
//!   call `draw` on the front and back, consequently. However, notice that this approach allows us to define
//!   various ways to compose different types that we will see in the following examples.

pub trait Draw {
    fn draw(&self);
}

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
#[allow(dead_code)]
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

fn dynamic_dispatch() {
    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        fn new() -> Self {
            Self { components: vec![] }
        }
        fn push(&mut self, component: Box<dyn Draw>) {
            self.components.push(component);
        }
        fn run(&self) {
            for c in &self.components {
                c.draw();
            }
        }
    }

    let mut screen = Screen::new();

    screen.push(Box::new(Button {
        width: 3,
        height: 4,
        label: String::from("login"),
    }));

    screen.push(Box::new(Button {
        width: 4,
        height: 5,
        label: String::from("logout"),
    }));

    screen.push(Box::new(SelectBox {
        width: 10,
        height: 6,
        options: vec![String::from("This"), String::from("that")],
    }));

    screen.run();

    // prints out:
    //
    // Button { width: 3, height: 4, label: "login" }
    // Button { width: 4, height: 5, label: "logout" }
    // SelectBox { width: 10, height: 6, options: ["This", "that"] }
}

fn static_dispatch() {
    use orx_meta::define_queue;

    define_queue!(
        elements => [Draw];
        queue => [Queue, NonEmptyQueue ; Empty, Single, Multi];
    );

    impl Draw for Empty {
        // describe what to do when there is no component to draw (identity)
        fn draw(&self) {}
    }
    impl<F: Draw> Draw for Single<F> {
        // describe what to do when there is one element to draw (trivial)
        fn draw(&self) {
            self.f.draw();
        }
    }
    impl<F: Draw, B: Queue> Draw for Multi<F, B> {
        // describe what to do when there are many elements to draw
        // this is where we define the composition!
        fn draw(&self) {
            self.f.draw();
            self.b.draw();
        }
    }

    pub struct Screen<Q: Queue>(Q);
    impl Screen<Empty> {
        fn new() -> Self {
            Self(Empty::new())
        }
    }
    impl<Q: Queue> Screen<Q> {
        fn push<S: Draw>(self, component: S) -> Screen<Q::PushBack<S>> {
            Screen(self.0.push(component))
        }
        fn run(&self) {
            self.0.draw();
        }
    }

    // let screen = Screen > ::default();

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
    //
    // Button { width: 3, height: 4, label: "login" }
    // Button { width: 4, height: 5, label: "logout" }
    // SelectBox { width: 10, height: 6, options: ["This", "that"] }
}

fn main() {
    println!("\ndynamic_dispatch");
    dynamic_dispatch();

    println!("\nstatic_dispatch");
    static_dispatch();
}
