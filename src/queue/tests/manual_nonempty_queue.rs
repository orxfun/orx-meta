// traits

trait Queue {
    type PushBack<X>: MultiQueue;

    type Front;

    type Back: Queue;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn front(&self) -> &Self::Front;

    fn into_front(self) -> Self::Front;
}

trait MultiQueue: Queue {
    fn pop_front(self) -> (Self::Front, Self::Back);
}

// impl: single

struct Single<F>(F);

impl<F> Queue for Single<F> {
    type PushBack<X> = Pair<F, Single<X>>;

    type Front = F;

    type Back = Self;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
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

struct Pair<F, B: Queue>(F, B);

impl<F, B: Queue> Queue for Pair<F, B> {
    type PushBack<X> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
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

impl<F, B: Queue> MultiQueue for Pair<F, B> {
    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}

// composition

struct QueueComposition;

impl QueueComposition {
    fn single<X>(x: X) -> Single<X> {
        Single(x)
    }

    fn compose<C: Queue, X>(q: C, x: X) -> C::PushBack<X> {
        q.push_back(x)
    }
}

// builder

struct Builder<Rem, Cur>(Cur, core::marker::PhantomData<Rem>)
where
    Rem: Queue,
    Cur: Queue;

impl<Rem, F> Builder<Rem, Single<F>>
where
    Rem: Queue,
{
    pub fn new(f: F) -> Self {
        Self(Single(f), core::marker::PhantomData)
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

    assert_eq!(x.front(), &String::from("xyz"));
    let (f, x) = x.pop_front();
    assert_eq!(f, String::from("xyz"));

    assert_eq!(x.front(), &true);
    let f = x.into_front();
    assert_eq!(f, true);
}
