use crate::MetaQueue;
use core::marker::PhantomData;

pub trait Builder {
    type Compose<X, Y>;
}

struct TupleBuilder0<M>
where
    M: MetaQueue,
{
    p: PhantomData<M>,
}

impl<M> TupleBuilder0<M>
where
    M: MetaQueue,
{
    pub fn compose(self, next: M::Front) -> TupleBuilder1<M::Back, M::Front> {
        TupleBuilder1 {
            p: PhantomData,
            t: next,
        }
    }
}

struct TupleBuilder1<M, T>
where
    M: MetaQueue,
{
    p: PhantomData<M>,
    t: T,
}

impl<M, T> TupleBuilder1<M, T>
where
    M: MetaQueue,
{
    pub fn compose(self, next: M::Front) -> TupleBuilder2<M::Back, T, M::Front> {
        TupleBuilder2 {
            p: PhantomData,
            data: (self.t, next),
        }
    }
}

struct TupleBuilder2<M, T, U>
where
    M: MetaQueue,
{
    p: PhantomData<M>,
    data: (T, U),
}

impl<M, T, U> TupleBuilder2<M, T, U>
where
    M: MetaQueue,
{
    pub fn compose(self, next: M::Front) -> TupleBuilder2<M::Back, T, (U, M::Front)> {
        TupleBuilder2 {
            p: PhantomData,
            data: (self.data.0, (self.data.1, next)),
        }
    }
}
