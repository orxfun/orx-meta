use orx_meta::define_queue;

// draw

pub trait Draw {
    fn draw(&self);
}

// example Draw components

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

// queue definition

define_queue!(
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

// screen as a queue

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

fn main() {
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
}
