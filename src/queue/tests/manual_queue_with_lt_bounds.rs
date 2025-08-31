use core::marker::PhantomData;

// bounds

// each item must be of Sth<'i> and Clone; we reduce them to marker trait Req<'i> (one trait & one lifetime)

trait Sth<'i> {} // actual requirement

trait Req<'i> {} // marker requirement that combines Sth<'i> and Clone

impl<'i, X> Req<'i> for X where X: Sth<'i> + Clone {}

// traits

trait Queue<'i> {
    type PushBack<X: Req<'i>>: NonEmptyQueue<'i>;

    type Front: Req<'i>;

    type Back: Queue<'i>;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

trait NonEmptyQueue<'i>: Queue<'i> {
    fn into_front(self) -> Self::Front;

    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// impl: empty

enum Never {}
impl<'i> Req<'i> for Never {}

#[derive(Default)]
struct EmptyQueue;

impl<'i> Queue<'i> for EmptyQueue {
    type PushBack<X: Req<'i>> = Single<'i, X>;

    type Front = Never;

    type Back = Self;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X> {
        Single(x, PhantomData)
    }

    fn len(&self) -> usize {
        0
    }
}

// impl: single

struct Single<'i, F: Req<'i>>(F, PhantomData<&'i ()>);

impl<'i, F: Req<'i>> Queue<'i> for Single<'i, F> {
    type PushBack<X: Req<'i>> = Pair<'i, F, Single<'i, X>>;

    type Front = F;

    type Back = EmptyQueue;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, Single(x, PhantomData), PhantomData)
    }

    fn len(&self) -> usize {
        1
    }
}

impl<'i, F: Req<'i>> NonEmptyQueue<'i> for Single<'i, F> {
    fn into_front(self) -> Self::Front {
        self.0
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }
}

// impl: pair

struct Pair<'i, F: Req<'i>, B: Queue<'i>>(F, B, PhantomData<&'i ()>);

impl<'i, F: Req<'i>, B: Queue<'i>> Queue<'i> for Pair<'i, F, B> {
    type PushBack<X: Req<'i>> = Pair<'i, F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X: Req<'i>>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, self.1.push_back(x), PhantomData)
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

impl<'i, F: Req<'i>, B: Queue<'i>> NonEmptyQueue<'i> for Pair<'i, F, B> {
    fn into_front(self) -> Self::Front {
        self.0
    }

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

    fn single<'i, X: Req<'i>>(x: X) -> Single<'i, X> {
        Single(x, PhantomData)
    }

    fn compose<'i, C: Queue<'i>, X: Req<'i>>(q: C, x: X) -> C::PushBack<X> {
        q.push_back(x)
    }
}

// builder

struct Builder<'i, Rem, Cur>(Cur, core::marker::PhantomData<&'i Rem>)
where
    Rem: Queue<'i>,
    Cur: Queue<'i>;

impl<'i, Rem> Builder<'i, Rem, EmptyQueue>
where
    Rem: Queue<'i>,
{
    pub fn new() -> Self {
        Self(EmptyQueue, core::marker::PhantomData)
    }
}

impl<'i, Rem, Cur> Builder<'i, Rem, Cur>
where
    Rem: Queue<'i>,
    Cur: Queue<'i>,
{
    fn push_back(self, x: Rem::Front) -> Builder<'i, Rem::Back, Cur::PushBack<Rem::Front>> {
        let current = self.0.push_back(x);
        Builder(current, core::marker::PhantomData)
    }

    fn finish(self) -> Cur
    where
        Rem: Queue<'i, Back = Rem>,
    {
        self.0
    }
}

// tests

impl<'i> Sth<'i> for char {}
impl<'i> Sth<'i> for i32 {}
impl<'i> Sth<'i> for String {}
impl<'i> Sth<'i> for bool {}

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

#[test]
fn builder() {
    type Target<'i> = Pair<'i, char, Pair<'i, i32, Pair<'i, String, Single<'i, bool>>>>;

    let builder = Builder::<Target, _>::new();

    let builder: Builder<Pair<i32, Pair<String, Single<bool>>>, Single<char>> =
        builder.push_back('x');

    let builder: Builder<Pair<String, Single<bool>>, Pair<char, Single<i32>>> =
        builder.push_back(32);

    let builder: Builder<Single<bool>, Pair<char, Pair<i32, Single<String>>>> =
        builder.push_back("xyz".to_string());

    let builder: Builder<EmptyQueue, Target> = builder.push_back(true);

    let x = builder.finish();

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
