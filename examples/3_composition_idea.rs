#![allow(dead_code)]

// draw

trait Draw {
    fn draw(&self);
}

// queue trait

trait StQueue: Draw {
    type PushBack<Elem>: StQueue
    where
        Elem: Draw;

    type Front: Draw;

    type Back: StQueue;

    const LEN: usize;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>
    where
        Elem: Draw;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn front(&self) -> &Self::Front;
}

// queue implementations: single

struct QueueSingle<F>
where
    F: Draw,
{
    front: F,
}

impl<F> StQueue for QueueSingle<F>
where
    F: Draw,
{
    type PushBack<Elem>
        = Queue<F, QueueSingle<Elem>>
    where
        Elem: Draw;

    type Front = F;

    type Back = Self;

    const LEN: usize = 1;

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>
    where
        Elem: Draw,
    {
        Queue {
            front: self.front,
            back: QueueSingle::new(element),
        }
    }

    fn front(&self) -> &Self::Front {
        &self.front
    }
}

impl<F> QueueSingle<F>
where
    F: Draw,
{
    /// Creates a new empty queue.
    pub fn new(element: F) -> Self {
        Self { front: element }
    }
}

impl<F> Draw for QueueSingle<F>
where
    F: Draw,
{
    fn draw(&self) {
        self.front.draw()
    }
}

// queue implementations: multiple

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

    fn front(&self) -> &Self::Front {
        &self.front
    }
}

impl<F> Queue<F, QueueSingle<F>>
where
    F: Draw,
{
    #[inline(always)]
    pub fn new(element: F) -> QueueSingle<F> {
        QueueSingle::new(element)
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

impl<F> Screen<QueueSingle<F>>
where
    F: Draw,
{
    fn new(element: F) -> Self {
        Self(QueueSingle::new(element))
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

// hand written

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

impl Queue<X1, Queue<X2, Queue<X3, QueueSingle<X4>>>> {
    // this is identical to Queue::draw
    fn draw_hand_written(&self) {
        self.front.draw(); // X1
        self.back.front.draw(); // X2
        self.back.back.front.draw(); // X3
        self.back.back.back.front.draw(); // X4
    }
}

fn main() {
    let screen = Screen::new(Button {
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
