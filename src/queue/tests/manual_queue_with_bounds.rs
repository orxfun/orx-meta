#![allow(dead_code)]

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

#[derive(Clone, Copy, Debug)]
enum Never {}
impl Req for Never {}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

// tuple support - 1

impl<X1> Single<X1>
where
    X1: Req,
{
    pub fn into_tuple(self) -> X1 {
        self.0
    }
}

impl<X1> From<X1> for Single<X1>
where
    X1: Req,
{
    fn from(x: X1) -> Self {
        Single(x)
    }
}

// tuple support - 2

impl<X1, X2> Pair<X1, Single<X2>>
where
    X1: Req,
    X2: Req,
{
    pub fn into_tuple(self) -> (X1, X2) {
        (self.0, self.1.0)
    }
}

impl<X1, X2> From<(X1, X2)> for Pair<X1, Single<X2>>
where
    X1: Req,
    X2: Req,
{
    fn from(x: (X1, X2)) -> Self {
        Single::from(x.0).push_back(x.1)
    }
}

// tuple support - 3

impl<X1, X2, X3> Pair<X1, Pair<X2, Single<X3>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.0, self.1.0, self.1.1.0)
    }
}

impl<X1, X2, X3> From<(X1, X2, X3)> for Pair<X1, Pair<X2, Single<X3>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
{
    fn from(x: (X1, X2, X3)) -> Self {
        Single::from(x.0).push_back(x.1).push_back(x.2)
    }
}

// tuple support - 4

impl<X1, X2, X3, X4> Pair<X1, Pair<X2, Pair<X3, Single<X4>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0)
    }
}

impl<X1, X2, X3, X4> From<(X1, X2, X3, X4)> for Pair<X1, Pair<X2, Pair<X3, Single<X4>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
{
    fn from(x: (X1, X2, X3, X4)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
    }
}

// tuple support - 5

impl<X1, X2, X3, X4, X5> Pair<X1, Pair<X2, Pair<X3, Pair<X4, Single<X5>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0, self.1.1.1.1.0)
    }
}

impl<X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)>
    for Pair<X1, Pair<X2, Pair<X3, Pair<X4, Single<X5>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
{
    fn from(x: (X1, X2, X3, X4, X5)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
            .push_back(x.4)
    }
}

// tuple support - 6

impl<X1, X2, X3, X4, X5, X6> Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Single<X6>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
        (
            self.0,
            self.1.0,
            self.1.1.0,
            self.1.1.1.0,
            self.1.1.1.1.0,
            self.1.1.1.1.1.0,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)>
    for Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Single<X6>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
{
    fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
            .push_back(x.4)
            .push_back(x.5)
    }
}

// tuple support - 7

impl<X1, X2, X3, X4, X5, X6, X7>
    Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Single<X7>>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
    X7: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
        (
            self.0,
            self.1.0,
            self.1.1.0,
            self.1.1.1.0,
            self.1.1.1.1.0,
            self.1.1.1.1.1.0,
            self.1.1.1.1.1.1.0,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)>
    for Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Single<X7>>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
    X7: Req,
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
            .push_back(x.4)
            .push_back(x.5)
            .push_back(x.6)
    }
}

// tuple support - 8

impl<X1, X2, X3, X4, X5, X6, X7, X8>
    Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Pair<X7, Single<X8>>>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
    X7: Req,
    X8: Req,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
        (
            self.0,
            self.1.0,
            self.1.1.0,
            self.1.1.1.0,
            self.1.1.1.1.0,
            self.1.1.1.1.1.0,
            self.1.1.1.1.1.1.0,
            self.1.1.1.1.1.1.1.0,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)>
    for Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Pair<X7, Single<X8>>>>>>>>
where
    X1: Req,
    X2: Req,
    X3: Req,
    X4: Req,
    X5: Req,
    X6: Req,
    X7: Req,
    X8: Req,
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
            .push_back(x.4)
            .push_back(x.5)
            .push_back(x.6)
            .push_back(x.7)
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

#[test]
fn tuple_support() {
    let t = ('x', 32, String::from("xyz"), true);
    let x: Pair<char, Pair<i32, Pair<String, Single<bool>>>> = t.clone().into();
    assert_eq!(x.clone().into_tuple(), t);
}
