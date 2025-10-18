#![allow(dead_code)]
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use repetitive::repetitive;

const SEED: u64 = 9562;

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
        Self {
            width,
            height,
            label,
        }
    }

    fn next(i: usize) -> Self {
        match i.is_multiple_of(2) {
            true => Self::new(i as u32 * 2, i as u32 + 10, i.to_string()),
            false => Self::new(i as u32, i as u32 + 5, i.to_string()),
        }
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
    fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self {
            width,
            height,
            options,
        }
    }
    fn next(i: usize) -> Self {
        match i.is_multiple_of(2) {
            true => Self::new(i as u32 * 2, i as u32 + 10, vec![]),
            false => Self::new(i as u32, i as u32 + 5, vec![i.to_string()]),
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}

enum Component {
    Button(Button),
    SelectBox(SelectBox),
}

impl Draw for Component {
    fn draw(&self) {
        match self {
            Self::Button(x) => x.draw(),
            Self::SelectBox(x) => x.draw(),
        }
    }
}

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

fn to_output(idx: &usize) -> u32 {
    let idx = *idx;
    fibonacci(&(idx as u32))
}

fn fibonacci(n: &u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..*n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

fn trait_object(n: usize) -> Vec<Box<dyn Draw>> {
    let mut vec: Vec<Box<dyn Draw>> = Vec::with_capacity(n);
    for i in 0..n {
        match i.is_multiple_of(3) {
            false => vec.push(Box::new(Button::next(i))),
            true => vec.push(Box::new(SelectBox::next(i))),
        }
    }
    vec
}

fn enum_component(n: usize) -> Vec<Component> {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        match i.is_multiple_of(3) {
            false => vec.push(Component::Button(Button::next(i))),
            true => vec.push(Component::SelectBox(SelectBox::next(i))),
        }
    }
    vec
}

fn queue() {
    let queue = EmptyScreen::new();
    // unroll! {
    //     for i in 0..2 {
    //         match i.is_multiple_of(3) {
    //             false => {
    //                 let queue = queue.push(Button::next(i));
    //             },
    //             true => {
    //                 let queue = queue.push(SelectBox::next(i));
    //             },
    //         }
    //     }
    // }

    repetitive! {
        @for i in 2..=4 {
            match (@i as usize).is_multiple_of(3) {
                false => {
                    let queue = queue.push(Button::next(@i));
                },
                true => {
                    let queue = queue.push(Button::next(@i));
                },
            }
        }
    }

    let q = queue;
}

fn run(c: &mut Criterion) {
    let treatments = [65_536 * 2];

    let mut group = c.benchmark_group("queue_build");

    for n in &treatments {
        // let input = inputs(*n);
        // let expected = seq(&input);

        // group.bench_with_input(BenchmarkId::new("seq", n), n, |b, _| {
        //     assert_eq!(&expected, &seq(&input));
        //     b.iter(|| seq(black_box(&input)))
        // });

        // group.bench_with_input(BenchmarkId::new("rayon", n), n, |b, _| {
        //     assert_eq!(&expected, &rayon(&input));
        //     b.iter(|| rayon(black_box(&input)))
        // });

        // group.bench_with_input(BenchmarkId::new("orx", n), n, |b, _| {
        //     assert_eq!(&expected, &orx(&input));
        //     b.iter(|| orx(black_box(&input)))
        // });
    }

    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
