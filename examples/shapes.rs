#![allow(dead_code)]

use orx_meta::define_queue;

define_queue!(
    lifetimes => [];
    generics => [];
    elements => [Draw];
    names => {
        traits: {
            queue: Queue,
            non_empty_queue: NonEmptyQueue,
        },
        structs: {
            empty: Empty,
            single: Single,
            pair: Pair,
            composition: QueueComposition,
            builder: Builder,
        },
    };
);

pub trait Draw {
    fn draw(&self);
}

pub struct Screen<Q: Queue> {
    shapes: Q,
}

impl Screen<Empty> {
    pub fn new() -> Self {
        Self {
            shapes: Empty::new(),
        }
    }
}

impl<Q: Queue> Screen<Q> {
    fn push<D: Draw>(self, shape: D) -> Screen<Q::PushBack<D>> {
        Screen {
            shapes: self.shapes.push_back(shape),
        }
    }

    fn run(&self) {
        self.shapes.draw();
    }
}

impl Draw for Empty {
    fn draw(&self) {}
}

impl<F: Draw> Draw for Single<F> {
    fn draw(&self) {
        self.f.draw();
    }
}

impl<F: Draw, B: Queue> Draw for Pair<F, B> {
    fn draw(&self) {
        self.f.draw();
        self.b.draw();
    }
}

// actual shapes

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        let (w, h) = (self.width, self.height);
        println!("Button({w}x{h})");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        let (w, h) = (self.width, self.height);
        println!("SelectBox({w}x{h})");
    }
}

fn main() {
    println!("\n\n# 1: screen.run()");
    let screen = Screen::new()
        .push(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        })
        .push(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        });

    screen.run();

    println!("\n\n# 2: screen.run()");
    let screen = screen.push(Button {
        width: 42,
        height: 42,
        label: String::from("42"),
    });
    screen.run();
}
