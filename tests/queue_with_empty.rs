use orx_meta::queue;

#[queue(Queue; Empty, Single, Multi)]
pub trait Fun {
    fn work(&self);

    fn work_with_number(&self, number: usize);
}

impl<X: Fn()> Fun for X {
    fn work(&self) {
        self();
    }

    fn work_with_number(&self, number: usize) {
        for _ in 0..number {
            self();
        }
    }
}

#[test]
fn forwards_methods_for_single_and_multi_queues() {
    let output = std::cell::RefCell::new(Vec::new());
    let first_output = &output;
    let second_output = &output;
    let queue = Single::new(move || first_output.borrow_mut().push("hey"))
        .push(move || second_output.borrow_mut().push("there"));

    queue.work();
    queue.work_with_number(2);

    assert_eq!(
        output.into_inner(),
        vec!["hey", "there", "hey", "hey", "there", "there"]
    );
}

#[test]
fn empty_queue_is_a_no_op_and_pushes_into_single() {
    let empty = Empty::new();
    empty.work();
    empty.work_with_number(3);

    let queue = empty.push(|| ());
    assert_eq!(queue.len(), 1);
}
