#![allow(dead_code)]

use orx_meta::define_queue_core;

define_queue_core!(
    elements => [Draw];
    names => {
        traits: { queue: DrawQueue, non_empty_queue: NonEmptyDrawQueue },
        structs: { empty: Empty, single: Single, pair: Pair }
    };
);

// behavior of elements
pub trait Draw {
    fn draw(&self);
}

impl Draw for Empty {
    fn draw(&self) {} // draw identity
}

impl<F: Draw> Draw for Single<F> {
    fn draw(&self) {
        self.f.draw(); // implementation of single is usually pretty standard
    }
}

impl<F: Draw, B: DrawQueue> Draw for Pair<F, B> {
    fn draw(&self) {
        // define composition over the Draw behavior
        self.f.draw();
        self.b.draw(); // notice that self.b.draw() is recursive since B itself is a queue
    }
}

// screen with convenient push method

pub struct Screen<Q: DrawQueue>(Q);

impl Screen<Empty> {
    pub fn new() -> Self {
        Self(Empty::new())
    }
}

impl<Q: DrawQueue> Screen<Q> {
    fn push<D: Draw>(self, shape: D) -> Screen<Q::PushBack<D>> {
        Screen(self.0.push_back(shape))
    }

    fn pop(self) -> (Screen<Q::Back>, Q::Front)
    where
        Q: NonEmptyDrawQueue,
    {
        let (f, b) = self.0.pop();
        (Screen(b), f)
    }

    fn drap_all_shapes(&self) {
        println!("=> Drawing {} shapes on the screen", self.0.len());
        self.0.draw();
        println!();
    }
}

fn main() {
    // some actual shape examples

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

    // init screen with chaned push calls

    let screen = Screen::new()
        .push(SelectBox {
            width: 75,
            height: 10,
            options: vec![String::from("Yes")],
        })
        .push(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        });
    screen.drap_all_shapes();

    // add two more shapes and pop the first shape in the front
    let screen = screen
        .push(Button {
            width: 42,
            height: 42,
            label: String::from("42"),
        })
        .push(SelectBox {
            width: 25,
            height: 30,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        });
    let (screen, front) = screen.pop();
    assert_eq!(front.options, vec![String::from("Yes"),]);

    screen.drap_all_shapes();
}
