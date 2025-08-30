// traits

trait Queue {
    type PushBack<X>: NonEmptyQueue;

    type Front;

    type Back: Queue;

    fn push_back<X>(self, x: X) -> Self::PushBack<X>;
}

trait NonEmptyQueue: Queue {
    fn front(&self) -> &Self::Front;

    fn pop_front(self) -> (Self::Front, Self::Back);
}

// empty

enum Never {}

struct EmptyQueue;

impl Queue for EmptyQueue {
    type PushBack<X> = Single<X>;

    type Front = Never;

    type Back = Self;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Single(x)
    }
}

// single

struct Single<F>(F);

impl<F> Queue for Single<F> {
    type PushBack<X> = Pair<F, Single<X>>;

    type Front = F;

    type Back = EmptyQueue;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        Pair(self.0, Single(x))
    }
}

impl<F> NonEmptyQueue for Single<F> {
    fn front(&self) -> &Self::Front {
        &self.0
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, EmptyQueue)
    }
}

// pair

struct Pair<F, B: Queue>(F, B);

impl<F, B: Queue> Queue for Pair<F, B> {
    type PushBack<X>;

    type Front;

    type Back;

    fn push_back<X>(self, x: X) -> Self::PushBack<X> {
        todo!()
    }
}

impl<F, B: Queue> NonEmptyQueue for Pair<F, B> {
    fn front(&self) -> &Self::Front {
        todo!()
    }

    fn pop_front(self) -> (Self::Front, Self::Back) {
        todo!()
    }
}
