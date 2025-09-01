#![allow(dead_code)]

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

    fn pop_front(self) -> (Self::Front, Self::Back);

    fn front(&self) -> &Self::Front;

    fn front_back(&self) -> (&Self::Front, &Self::Back);
}

// impl: empty

#[derive(Clone, Copy, Debug)]
enum Never {}
impl<'i> Req<'i> for Never {}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug)]
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

// tuple support - 1

impl<'i, X1> Single<'i, X1>
where
    X1: Req<'i>,
{
    pub fn into_tuple(self) -> X1 {
        self.0
    }
}

impl<'i, X1> From<X1> for Single<'i, X1>
where
    X1: Req<'i>,
{
    fn from(x: X1) -> Self {
        Single(x, Default::default())
    }
}

// tuple support - 2

impl<'i, X1, X2> Pair<'i, X1, Single<'i, X2>>
where
    X1: Req<'i>,
    X2: Req<'i>,
{
    pub fn into_tuple(self) -> (X1, X2) {
        (self.0, self.1.0)
    }
}

impl<'i, X1, X2> From<(X1, X2)> for Pair<'i, X1, Single<'i, X2>>
where
    X1: Req<'i>,
    X2: Req<'i>,
{
    fn from(x: (X1, X2)) -> Self {
        Single::from(x.0).push_back(x.1)
    }
}

// tuple support - 3

impl<'i, X1, X2, X3> Pair<'i, X1, Pair<'i, X2, Single<'i, X3>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
{
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.0, self.1.0, self.1.1.0)
    }
}

impl<'i, X1, X2, X3> From<(X1, X2, X3)> for Pair<'i, X1, Pair<'i, X2, Single<'i, X3>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
{
    fn from(x: (X1, X2, X3)) -> Self {
        Single::from(x.0).push_back(x.1).push_back(x.2)
    }
}

// tuple support - 4

impl<'i, X1, X2, X3, X4> Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Single<'i, X4>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0)
    }
}

impl<'i, X1, X2, X3, X4> From<(X1, X2, X3, X4)>
    for Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Single<'i, X4>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
{
    fn from(x: (X1, X2, X3, X4)) -> Self {
        Single::from(x.0)
            .push_back(x.1)
            .push_back(x.2)
            .push_back(x.3)
    }
}

// tuple support - 5

impl<'i, X1, X2, X3, X4, X5> Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Single<'i, X5>>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0, self.1.1.1.1.0)
    }
}

impl<'i, X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)>
    for Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Single<'i, X5>>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6>
    Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Single<'i, X6>>>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)>
    for Pair<'i, X1, Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Single<'i, X6>>>>>>
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6, X7>
    Pair<
        'i,
        X1,
        Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Pair<'i, X6, Single<'i, X7>>>>>>,
    >
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
    X7: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)>
    for Pair<
        'i,
        X1,
        Pair<'i, X2, Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Pair<'i, X6, Single<'i, X7>>>>>>,
    >
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
    X7: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6, X7, X8>
    Pair<
        'i,
        X1,
        Pair<
            'i,
            X2,
            Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Pair<'i, X6, Pair<'i, X7, Single<'i, X8>>>>>>,
        >,
    >
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
    X7: Req<'i>,
    X8: Req<'i>,
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

impl<'i, X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)>
    for Pair<
        'i,
        X1,
        Pair<
            'i,
            X2,
            Pair<'i, X3, Pair<'i, X4, Pair<'i, X5, Pair<'i, X6, Pair<'i, X7, Single<'i, X8>>>>>>,
        >,
    >
where
    X1: Req<'i>,
    X2: Req<'i>,
    X3: Req<'i>,
    X4: Req<'i>,
    X5: Req<'i>,
    X6: Req<'i>,
    X7: Req<'i>,
    X8: Req<'i>,
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
