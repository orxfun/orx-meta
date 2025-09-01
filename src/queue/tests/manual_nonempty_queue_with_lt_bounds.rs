// bounds

// each item must be of Sth<'i> and Clone; we reduce them to marker trait Req<'i> (one trait & one lifetime)

trait Sth<'i> {} // actual requirement

trait Req<'i> {} // marker requirement that combines Sth<'i> and Clone

impl<'i, X> Req<'i> for X where X: Sth<'i> + Clone {}

// traits

trait Queue<'i> {
    type PushBack<X: Req<'i>>: MultiQueue<'i>;

    type Front: Req<'i>;

    type Back: Queue<'i>;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn front(&self) -> &Self::Front;

    fn into_front(self) -> Self::Front;
}

trait MultiQueue<'i>: Queue<'i> {
    fn front_back(&self) -> (&Self::Front, &Self::Back);

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// impl: single

struct Single<'i, F: Req<'i>>(F, core::marker::PhantomData<&'i ()>);

impl<'i, F: Req<'i>> Queue<'i> for Single<'i, F> {
    type PushBack<X: Req<'i>> = Pair<'i, F, Single<'i, X>>;

    type Front = F;

    type Back = Self;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X> {
        Pair(
            self.0,
            Single(x, core::marker::PhantomData),
            core::marker::PhantomData,
        )
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

struct Pair<'i, F: Req<'i>, B: Queue<'i>>(F, B, core::marker::PhantomData<&'i ()>);

impl<'i, F: Req<'i>, B: Queue<'i>> Queue<'i> for Pair<'i, F, B> {
    type PushBack<X: Req<'i>> = Pair<'i, F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, self.1.push_back(x), core::marker::PhantomData)
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

impl<'i, F: Req<'i>, B: Queue<'i>> MultiQueue<'i> for Pair<'i, F, B> {
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
    fn single<'i, X: Req<'i>>(x: X) -> Single<'i, X> {
        Single(x, core::marker::PhantomData)
    }

    fn compose<'i, C: Queue<'i>, X: Req<'i>>(q: C, x: X) -> C::PushBack<X> {
        q.push_back(x)
    }
}

// tests

impl<'i> Req<'i> for char {}
impl<'i> Req<'i> for i32 {}
impl<'i> Req<'i> for String {}
impl<'i> Req<'i> for bool {}

#[test]
fn one() {
    let x = Single('x', core::marker::PhantomData);

    assert_eq!(x.front(), &'x');
    let f = x.into_front();
    assert_eq!(f, 'x');
}

#[test]
fn two() {
    let x = Single('x', core::marker::PhantomData);
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
    let x = Single('x', core::marker::PhantomData);
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
    let x = Single('x', core::marker::PhantomData);
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
