// draw

trait Draw {
    fn draw(&self);
}

// queue trait

trait StQueue: Draw {
    type PushBack<T>: StQueue
    where
        T: Draw;

    type Front: Draw;

    type Back: StQueue;

    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw;
}

// queue implementations: empty

struct EmptyQueue;

impl StQueue for EmptyQueue {
    type PushBack<T>
        = Queue<T, EmptyQueue>
    where
        T: Draw;

    type Front = Self;

    type Back = Self;

    const LEN: usize = 0;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw,
    {
        Queue::new(element)
    }
}

impl EmptyQueue {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self
    }
}

impl Draw for EmptyQueue {
    fn draw(&self) {}
}

// queue implementations: non-empty

struct Queue<F, B>
where
    F: Draw,
    B: StQueue,
{
    front: F,
    back: B,
}

impl<F, B> StQueue for Queue<F, B>
where
    F: Draw,
    B: StQueue,
{
    type PushBack<T>
        = Queue<F, B::PushBack<T>>
    where
        T: Draw;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    fn push<T>(self, element: T) -> Self::PushBack<T>
    where
        T: Draw,
    {
        Queue {
            front: self.front,
            back: self.back.push(element),
        }
    }
}

impl<F> Queue<F, EmptyQueue>
where
    F: Draw,
{
    pub fn new(front: F) -> Self {
        Self {
            front,
            back: EmptyQueue,
        }
    }
}

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

pub struct X1;
impl Draw for X1 {
    fn draw(&self) {}
}
pub struct X2;
impl Draw for X2 {
    fn draw(&self) {}
}
pub struct X3;
impl Draw for X3 {
    fn draw(&self) {}
}
pub struct X4;
impl Draw for X4 {
    fn draw(&self) {}
}

impl Queue<X1, Queue<X2, Queue<X3, Queue<X4, EmptyQueue>>>> {
    fn draw(&self) {
        self.front.draw(); // X1
        self.back.front.draw(); // X2
        self.back.back.front.draw(); // X3
        self.back.back.back.front.draw(); // X4
        self.back.back.back.back.draw(); // EmptyQueue
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
