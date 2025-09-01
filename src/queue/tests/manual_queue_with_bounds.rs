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
    fn into_front(self) -> Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);

    fn front(&self) -> &Self::Front;

    fn front_back(&self) -> (&Self::Front, &Self::Back);
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
    fn into_front(self) -> Self::Front {
        self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn front_back(&self) -> (&Self::Front, &Self::Back) {
        (&self.0, &EmptyQueue)
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
    fn into_front(self) -> Self::Front {
        self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }

    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn front_back(&self) -> (&Self::Front, &Self::Back) {
        (&self.0, &self.1)
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

// builder

struct Builder<Rem, Cur>(Cur, core::marker::PhantomData<Rem>)
where
    Rem: Queue,
    Cur: Queue;

impl<Rem> Builder<Rem, EmptyQueue>
where
    Rem: Queue,
{
    pub fn new() -> Self {
        Self(EmptyQueue, core::marker::PhantomData)
    }
}

impl<Rem, Cur> Builder<Rem, Cur>
where
    Rem: Queue,
    Cur: Queue,
{
    fn push_back(self, x: Rem::Front) -> Builder<Rem::Back, Cur::PushBack<Rem::Front>> {
        let current = self.0.push_back(x);
        Builder(current, core::marker::PhantomData)
    }

    fn finish(self) -> Cur
    where
        Rem: Queue<Back = Rem>,
    {
        self.0
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

#[test]
fn builder() {
    type Target = Pair<char, Pair<i32, Pair<String, Single<bool>>>>;

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
    let (f, b) = x.front_back();
    assert_eq!(f, &true);
    assert!(b.is_empty());
    let f = x.into_front();
    assert_eq!(f, true);
}
