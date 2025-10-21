pub trait Sum {
    fn sum(self) -> i64;
}

impl Sum for i16 {
    fn sum(self) -> i64 {
        self as i64
    }
}

impl Sum for i32 {
    fn sum(self) -> i64 {
        self as i64
    }
}

impl Sum for i64 {
    fn sum(self) -> i64 {
        self
    }
}

orx_meta::define_nonempty_queue!(
    elements => [ Sum ];
    queue => [ StQueue ; QueueSingle, Queue ];
);

impl<F: Sum> Sum for QueueSingle<F> {
    fn sum(self) -> i64 {
        self.f.sum()
    }
}

impl<F: Sum, B: StQueue> Sum for Queue<F, B> {
    fn sum(self) -> i64 {
        self.f.sum() + self.b.sum()
    }
}

fn main() {
    let queue = QueueSingle::new(1i16)
        .push(2i32)
        .push(3i32)
        .push(4i64)
        .push(5i32)
        .push(6i64)
        .push(7i16);
    let sum = queue.sum();
    assert_eq!(sum, 28);
}
