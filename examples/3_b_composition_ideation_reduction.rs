use orx_meta::define_queue;

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

define_queue!(
    elements => [ Sum ];
    queue => [ StQueue ; EmptyQueue, Queue ];
);

impl Sum for EmptyQueue {
    fn sum(self) -> i64 {
        0
    }
}

impl<F: Sum, B: StQueue> Sum for Queue<F, B> {
    fn sum(self) -> i64 {
        self.f.sum() + self.b.sum()
    }
}

fn main() {
    let queue = EmptyQueue::new()
        .push(1i16)
        .push(2i32)
        .push(3i32)
        .push(4i64)
        .push(5i32)
        .push(6i64)
        .push(7i16);
    let sum = queue.sum();
    assert_eq!(sum, 28);
}
