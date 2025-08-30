use crate::Never;

pub trait MyTrait {}

pub trait Queue {
    type PushBack<X: MyTrait>: NonEmptyQueue;

    type Front: MyTrait;

    type Back: Queue;

    fn push_back<X: MyTrait>(self, x: X) -> Self::PushBack<X>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait NonEmptyQueue: Queue {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// empty

impl MyTrait for Never {}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyQueue;

impl Queue for EmptyQueue {
    type PushBack<X: MyTrait> = Single<X>;

    type Front = Never;

    type Back = EmptyQueue;

    fn push_back<X: MyTrait>(self, x: X) -> Self::PushBack<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}

// single

#[derive(Debug, Clone, Copy)]
pub struct Single<T>(pub(super) T)
where
    T: MyTrait;

impl<T> Queue for Single<T>
where
    T: MyTrait,
{
    type PushBack<X: MyTrait> = Pair<T, Single<X>>;

    type Front = T;

    type Back = EmptyQueue;

    fn push_back<X: MyTrait>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, Single(x))
    }

    fn len(&self) -> usize {
        1
    }
}

impl<T> NonEmptyQueue for Single<T>
where
    T: MyTrait,
{
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }
}

// pair

#[derive(Debug, Clone, Copy)]
pub struct Pair<F, B>(pub(super) F, pub(super) B)
where
    F: MyTrait;

impl<F, B> Queue for Pair<F, B>
where
    F: MyTrait,
    B: Queue,
{
    type PushBack<X: MyTrait> = Pair<F, B::PushBack<X>>;

    type Front = F;

    type Back = B;

    fn push_back<X: MyTrait>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, self.1.push_back(x))
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

impl<F, B> NonEmptyQueue for Pair<F, B>
where
    F: MyTrait,
    B: Queue,
{
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}
