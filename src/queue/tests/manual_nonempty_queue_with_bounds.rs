#![allow(dead_code)]

// bounds

trait Req {}

// traits

trait Queue {
    type PushBack<X: Req>: MultiQueue;

    type Front: Req;

    type Back: Queue;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn front(&self) -> &Self::Front;

    fn into_front(self) -> Self::Front;
}

trait MultiQueue: Queue {
    fn front_back(&self) -> (&Self::Front, &Self::Back);

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// impl: single

struct Single<F: Req>(F);

impl<F: Req> Queue for Single<F> {
    type PushBack<X: Req> = Pair<F, Single<X>>;

    type Front = F;

    type Back = Self;

    fn push_back<X: Req>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, Single(x))
    }

    fn len(&self) -> usize {
        1
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn into_front(self) -> Self::Front {
        self.0
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

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn into_front(self) -> Self::Front {
        self.0
    }
}

impl<F: Req, B: Queue> MultiQueue for Pair<F, B> {
    fn front_back(&self) -> (&Self::Front, &Self::Back) {
        (&self.0, &self.1)
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}

// composition

struct QueueComposition;

impl QueueComposition {
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
    let x = Single('x');

    assert_eq!(x.front(), &'x');
    let f = x.into_front();
    assert_eq!(f, 'x');
}

#[test]
fn two() {
    let x = Single('x');
    let x = x.push_back(32);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let f = x.into_front();
    assert_eq!(f, 32);
}

#[test]
fn three() {
    let x = Single('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    assert_eq!(x.front(), &String::from("xyz"));
    let f = x.into_front();
    assert_eq!(f, String::from("xyz"));
}

#[test]
fn four() {
    let x = Single('x');
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
    let f = x.into_front();
    assert_eq!(f, true);
}

#[test]
fn compose_four() {
    type C = QueueComposition;

    let x = C::single('x');
    let x = C::compose(x, 32);
    let x = C::compose(x, String::from("xyz"));
    let x = C::compose(x, true);

    assert_eq!(x.len(), 4);

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert_eq!(x.front(), &32);
    let (f, x) = x.pop_front();
    assert_eq!(f, 32);

    let (f, b) = x.front_back();
    assert_eq!(f, &String::from("xyz"));
    assert_eq!(b.len(), 1);
    assert_eq!(b.front(), &true);

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let f = x.into_front();
    assert_eq!(f, true);
}
