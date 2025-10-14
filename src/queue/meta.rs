pub trait QueuePush {
    type PushBack<X>: QueuePush;

    type PushFront<X>: QueuePush;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push_back<X>(self, element: X) -> Self::PushBack<X>;

    fn push_front<X>(self, element: X) -> Self::PushFront<X>;
}

pub trait NonEmptyQueue: QueuePush {
    type Front;

    type Back;

    type PopFront;

    type PopBack;

    fn pop_back(self) -> (Self::Back, Self::PopBack);

    fn pop_front(self) -> (Self::Front, Self::PopFront);
}

struct Empty;

impl QueuePush for Empty {
    type PushBack<X> = Single<X>;

    type PushFront<X> = Single<X>;

    fn len(&self) -> usize {
        0
    }

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        todo!()
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        todo!()
    }
}

struct Single<T> {
    element: T,
}

impl<T> QueuePush for Single<T> {
    type PushBack<B> = Multi<T, Empty, B>;

    type PushFront<F> = Multi<F, Empty, T>;

    fn len(&self) -> usize {
        1
    }

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        todo!()
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        todo!()
    }
}

impl<T> NonEmptyQueue for Single<T> {
    type Front = T;

    type PopFront = Empty;

    type Back = T;

    type PopBack = Empty;

    fn pop_back(self) -> (Self::Back, Self::PopBack) {
        todo!()
    }

    fn pop_front(self) -> (Self::Front, Self::PopFront) {
        todo!()
    }
}

struct Multi<F, M, B>
where
    M: QueuePush,
{
    front: F,
    middle: M,
    back: B,
}

impl<F, M: QueuePush, B> QueuePush for Multi<F, M, B> {
    type PushBack<X> = Multi<F, M::PushBack<B>, X>;

    type PushFront<X> = Multi<X, M::PushFront<F>, B>;

    fn len(&self) -> usize {
        2 + self.middle.len()
    }

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        todo!()
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        todo!()
    }
}

impl<F, M: QueuePush, B> NonEmptyQueue for Multi<F, M, B> {
    type Front = F;

    type PopFront = M::PushBack<B>;

    type Back = B;

    type PopBack = M::PushFront<F>;

    fn pop_back(self) -> (Self::Back, Self::PopBack) {
        todo!()
    }

    fn pop_front(self) -> (Self::Front, Self::PopFront) {
        todo!()
    }
}

#[test]
fn abc() {
    let q = Empty;
    let q = q.push_back(1);
    let q = q.push_back(true);
    let q = q.push_back('x');
    let q = q.push_back("foo");

    let (x, q) = q.pop_back();
    let (x, q) = q.pop_back();
    let (x, q) = q.pop_back();
    let (x, q) = q.pop_back();
}
