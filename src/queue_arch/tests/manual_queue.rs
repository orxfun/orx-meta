#![allow(dead_code)]

trait Queue {
    // traits
    type PushBack<X>: NonEmptyQueue;

    type Front;

    type Back: Queue;

    type Elevated: Queue;

    fn push<X>(self, x: impl Into<Single<X>>) -> Self::PushBack<X>;

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

#[derive(Clone, Copy, Debug, Default)]
struct EmptyQueue;

impl Queue for EmptyQueue {
    type PushBack<X> = Single<X>;

    type Front = Never;

    type Back = Self;

    type Elevated = EmptyQueue;

    fn push<X>(self, x: impl Into<Single<X>>) -> Self::PushBack<X> {
        x.into()
    }

    fn len(&self) -> usize {
        0
    }
}

// impl: single

#[derive(Clone, Copy, Debug)]
struct Single<F>(F);

impl<F> Queue for Single<F> {
    type PushBack<X> = Pair<F, Single<X>>;

    type Front = F;

    type Back = EmptyQueue;

    type Elevated = Single<Single<F>>;

    fn push<X>(self, x: impl Into<Single<X>>) -> Self::PushBack<X> {
        Pair(self.0, x.into())
    }

    fn len(&self) -> usize {
        1
    }
}

impl<F> NonEmptyQueue for Single<F> {
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
struct Pair<F, B: Queue>(F, B);

impl<F, B: Queue> Queue for Pair<F, B> {
    type PushBack<X> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    type Elevated = Pair<Single<F>, B::Elevated>;

    fn push<X>(self, x: impl Into<Single<X>>) -> Self::PushBack<X> {
        Pair(self.0, self.1.push(x))
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

impl<F, B: Queue> NonEmptyQueue for Pair<F, B> {
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

struct QueueComposition;

impl QueueComposition {
    fn empty() -> EmptyQueue {
        Default::default()
    }

    fn single<X>(x: X) -> Single<X> {
        Single(x)
    }

    fn compose<C: Queue, X>(q: C, x: X) -> C::PushBack<X> {
        q.push(x)
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
    fn push(self, x: Rem::Front) -> Builder<Rem::Back, Cur::PushBack<Rem::Front>> {
        let current = self.0.push(x);
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

impl<X1> Single<X1> {
    pub fn into_tuple(self) -> X1 {
        self.0
    }
}

impl<X1> From<X1> for Single<X1> {
    fn from(x: X1) -> Self {
        Single(x)
    }
}

// tuple support - 2

impl<X1, X2> Pair<X1, Single<X2>> {
    pub fn into_tuple(self) -> (X1, X2) {
        (self.0, self.1.0)
    }
}

impl<X1, X2> From<(X1, X2)> for Pair<X1, Single<X2>> {
    fn from(x: (X1, X2)) -> Self {
        Single::from(x.0).push(x.1)
    }
}

// tuple support - 3

impl<X1, X2, X3> Pair<X1, Pair<X2, Single<X3>>> {
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.0, self.1.0, self.1.1.0)
    }
}

impl<X1, X2, X3> From<(X1, X2, X3)> for Pair<X1, Pair<X2, Single<X3>>> {
    fn from(x: (X1, X2, X3)) -> Self {
        Single::from(x.0).push(x.1).push(x.2)
    }
}

// tuple support - 4

impl<X1, X2, X3, X4> Pair<X1, Pair<X2, Pair<X3, Single<X4>>>> {
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0)
    }
}

impl<X1, X2, X3, X4> From<(X1, X2, X3, X4)> for Pair<X1, Pair<X2, Pair<X3, Single<X4>>>> {
    fn from(x: (X1, X2, X3, X4)) -> Self {
        Single::from(x.0)
            .push(x.1)
            .push(x.2)
            .push(x.3)
    }
}

// tuple support - 5

impl<X1, X2, X3, X4, X5> Pair<X1, Pair<X2, Pair<X3, Pair<X4, Single<X5>>>>> {
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (self.0, self.1.0, self.1.1.0, self.1.1.1.0, self.1.1.1.1.0)
    }
}

impl<X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)>
    for Pair<X1, Pair<X2, Pair<X3, Pair<X4, Single<X5>>>>>
{
    fn from(x: (X1, X2, X3, X4, X5)) -> Self {
        Single::from(x.0)
            .push(x.1)
            .push(x.2)
            .push(x.3)
            .push(x.4)
    }
}

// tuple support - 6

impl<X1, X2, X3, X4, X5, X6> Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Single<X6>>>>>> {
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
{
    fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
        Single::from(x.0)
            .push(x.1)
            .push(x.2)
            .push(x.3)
            .push(x.4)
            .push(x.5)
    }
}

// tuple support - 7

impl<X1, X2, X3, X4, X5, X6, X7>
    Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Single<X7>>>>>>>
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
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
        Single::from(x.0)
            .push(x.1)
            .push(x.2)
            .push(x.3)
            .push(x.4)
            .push(x.5)
            .push(x.6)
    }
}

// tuple support - 8

impl<X1, X2, X3, X4, X5, X6, X7, X8>
    Pair<X1, Pair<X2, Pair<X3, Pair<X4, Pair<X5, Pair<X6, Pair<X7, Single<X8>>>>>>>>
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
{
    fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
        Single::from(x.0)
            .push(x.1)
            .push(x.2)
            .push(x.3)
            .push(x.4)
            .push(x.5)
            .push(x.6)
            .push(x.7)
    }
}

// tests

#[test]
fn one() {
    let x = EmptyQueue;
    let x = x.push('x');

    assert_eq!(x.front(), &'x');
    let (f, x) = x.pop_front();
    assert_eq!(f, 'x');

    assert!(x.is_empty());
}

#[test]
fn two() {
    let x = EmptyQueue;
    let x = x.push('x');
    let x = x.push(32);

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
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));

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
    let x = x.push('x');
    let x = x.push(32);
    let x = x.push(String::from("xyz"));
    let x = x.push(true);

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
fn composition() {
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
        builder.push('x');

    let builder: Builder<Pair<String, Single<bool>>, Pair<char, Single<i32>>> =
        builder.push(32);

    let builder: Builder<Single<bool>, Pair<char, Pair<i32, Single<String>>>> =
        builder.push("xyz".to_string());

    let builder: Builder<EmptyQueue, Target> = builder.push(true);

    let x = builder.finish();
    assert_eq!(x.len(), 4);

    // alternatively

    type QueueOf<A, B, C, D> = Pair<A, Pair<B, Pair<C, Single<D>>>>;

    type Target2 = QueueOf<char, i32, String, bool>;

    let x = Builder::<Target2, _>::new()
        .push('x')
        .push(32)
        .push("xyz".to_string())
        .push(true)
        .finish();
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
