// bounds

trait Req {}

// traits

trait Queue {
    type PushBack<X: Req>: NonEmptyQueue;

    type Front: Req;

    type Back: Queue;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

trait NonEmptyQueue: Queue {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// impl: empty

enum Never {}
impl Req for Never {}

#[derive(Default)]
struct EmptyQueue;

impl Queue for EmptyQueue {
    type PushBack<X: Req> = Single<X>;

    type Front = Never;

    type Back = Self;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}

// impl: single

struct Single<F: Req>(F);

impl<F: Req> Queue for Single<F> {
    type PushBack<X: Req> = Pair<F, Single<X>>;

    type Front = F;

    type Back = EmptyQueue;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, Single(x))
    }

    fn len(&self) -> usize {
        1
    }
}

impl<F: Req> NonEmptyQueue for Single<F> {
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }
}

// impl: pair

struct Pair<F: Req, B: Queue>(F, B);

impl<F: Req, B: Queue> Queue for Pair<F, B> {
    type PushBack<X: Req> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, self.1.push_back(x))
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

impl<F: Req, B: Queue> NonEmptyQueue for Pair<F, B> {
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}

// composition

#[derive(Clone, Copy, Default)]
struct QueueComposition;

impl QueueComposition {
    fn empty() -> EmptyQueue {
        Default::default()
    }

    fn single<X: Req>(x: X) -> Single<X> {
        Single(x)
    }

    fn compose<C: Queue, X: Req>(q: C, x: X) -> C::PushBack<X> {
        q.push_back(x)
    }
}

// tests

impl Req for char {}
impl Req for i32 {}
impl Req for String {}
impl Req for bool {}

#[test]
fn one() {
    let x = EmptyQueue;
    let x = x.push_back('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn two() {
    let x = EmptyQueue;
    let x = x.push_back('x');
    let x = x.push_back(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert!(x.is_empty());
}

#[test]
fn three() {
    let x = EmptyQueue;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert!(x.is_empty());
}

#[test]
fn four() {
    let x = EmptyQueue;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop_front();
    assert_eq!(f, true);

    assert!(x.is_empty());
}

#[test]
fn compose_four() {
    type C = QueueComposition;

    let x = C::empty();
    let x = C::compose(x, 'x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop_front();
    assert_eq!(f, true);

    assert!(x.is_empty());

    let x = C::single('x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let (f, x) = x.pop_front();
    assert_eq!(f, true);

    assert!(x.is_empty());
}
