use orx_meta::queue;
use std::cell::RefCell;

// A simplified Scope trait for testing generic method forwarding
pub trait Scope<'s, 'env, 'scope>: Copy {
    fn run<W>(self, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: FnOnce() + 'scope + 'env;
}

// A simple scope implementation that immediately runs work
#[derive(Clone, Copy)]
pub struct ImmediateScope;

impl<'s, 'env, 'scope> Scope<'s, 'env, 'scope> for ImmediateScope {
    fn run<W>(self, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: FnOnce() + 'scope + 'env,
    {
        work();
    }
}

#[queue(TaskQueue; TasksSingle, TasksMulti)]
pub trait ParFun {
    fn run<'s, 'env, 'scope>(self, scope: impl Scope<'s, 'env, 'scope>)
    where
        'scope: 's,
        'env: 'scope + 's,
        Self: 'scope + 'env;
}

impl<F: Fn()> ParFun for F {
    fn run<'s, 'env, 'scope>(self, scope: impl Scope<'s, 'env, 'scope>)
    where
        'scope: 's,
        'env: 'scope + 's,
        Self: 'scope + 'env,
    {
        scope.run(self);
    }
}

#[test]
fn forwards_generic_methods_for_single_and_multi_queues() {
    let output = RefCell::new(Vec::new());
    let first_output = &output;
    let second_output = &output;

    let queue = TasksSingle::new(move || first_output.borrow_mut().push("first"))
        .push(move || second_output.borrow_mut().push("second"));

    queue.run(ImmediateScope);

    assert_eq!(output.into_inner(), vec!["first", "second"]);
}

#[test]
fn empty_queue_is_a_no_op_with_generic_methods() {
    let output = RefCell::new(Vec::new());
    let output_ref = &output;

    // Simple test with a single queue element
    let queue: TasksSingle<_> = TasksSingle::new(move || output_ref.borrow_mut().push("work"));
    queue.run(ImmediateScope);

    assert_eq!(output.into_inner(), vec!["work"]);
}
