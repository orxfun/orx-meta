use orx_meta::define_queue;
use std::fmt::Display;

define_queue!(
    elements => [Display];
    names => {
        traits: { queue: DisplayQueue, non_empty_queue: NonEmptyDisplayQueue },
        structs: { empty: Empty, single: Single, pair: Pair }
    };
);

// implement required bound on queue implementations

impl Display for Empty {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(()) // display identity
    }
}

impl<F: Display> Display for Single<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // implementation of single is usually pretty standard
        write!(f, "{}", self.f)
    }
}

impl<F: Display, B: DisplayQueue> Display for Pair<F, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // define composition
        // notice that self.b formatting is recursive since B itself is a queue
        write!(f, "{}-{}", self.f, self.b)
    }
}

fn main() {
    // grow

    let q = Empty::new();
    assert_eq!(q.to_string(), "");

    let q = q.push_back('x');
    assert_eq!(q.to_string(), "x");

    let q = q.push_back(12);
    assert_eq!(q.to_string(), "x-12");

    let q = q.push_back(true);
    assert_eq!(q.to_string(), "x-12-true");

    // shrink

    let (f, q) = q.pop();
    assert_eq!(f, 'x');
    assert_eq!(q.to_string(), "12-true");

    let (f, q) = q.pop();
    assert_eq!(f, 12);
    assert_eq!(q.to_string(), "true");

    let (f, q) = q.pop();
    assert_eq!(f, true);
    assert_eq!(q.to_string(), "");

    // chain

    let str = Empty::new()
        .push_back('x')
        .push_back(12)
        .push_back(true)
        .to_string();
    assert_eq!(str, "x-12-true");
}
