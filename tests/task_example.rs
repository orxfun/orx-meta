// Exact example from the task documentation
use orx_meta::queue;

// Minimal Scope trait definition for this test
pub trait Scope<'s, 'env, 'scope>: Copy {
    fn run<W>(self, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: FnOnce() + 'scope + 'env;
}

#[derive(Clone, Copy)]
pub struct TestScope;

impl<'s, 'env, 'scope> Scope<'s, 'env, 'scope> for TestScope {
    fn run<W>(self, work: W)
    where
        'scope: 's,
        'env: 'scope + 's,
        W: FnOnce() + 'scope + 'env,
    {
        work();
    }
}

// The exact example from docs/task.md - now it should work!
#[queue(TaskQueue; TasksSingle, TasksMulti)]
pub trait ParFun {
    fn run<'s, 'env, 'scope>(self, scope: impl Scope<'s, 'env, 'scope>)
    where
        'scope: 's,
        'env: 'scope + 's,
        Self: 'scope + 'env;
}

// Implement ParFun for a simple closure type
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
fn task_example_compiles_and_works() {
    use std::cell::RefCell;

    let output = RefCell::new(Vec::new());
    let output_ref = &output;

    let queue = TasksSingle::new(move || {
        output_ref.borrow_mut().push("task completed");
    });

    queue.run(TestScope);

    assert_eq!(output.into_inner(), vec!["task completed"]);
}
