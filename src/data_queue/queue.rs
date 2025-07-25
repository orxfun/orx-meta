use crate::Never;

pub trait DataQueue {
    type Push<X>: DataQueue;

    type Front;

    type Back: DataQueue;

    fn push<X>(self, x: X) -> Self::Push<X>;

    fn pop(self) -> (Self::Front, Self::Back);
}

struct Empty;
impl DataQueue for Empty {
    type Push<X> = Single<X>;

    type Front = Never;

    type Back = Empty;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Single(x)
    }

    fn pop(self) -> (Self::Front, Self::Back) {
        todo!()
    }
}

struct Single<T>(T);
impl<T> DataQueue for Single<T> {
    type Push<X> = Pair<T, Single<X>>;

    type Front = T;

    type Back = Empty;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, Single(x))
    }

    fn pop(self) -> (Self::Front, Self::Back) {
        (self.0, Empty)
    }
}

struct Pair<F, B>(F, B);
impl<F, B> DataQueue for Pair<F, B>
where
    B: DataQueue,
{
    type Push<X> = Pair<F, B::Push<X>>;

    type Front = F;

    type Back = B;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, self.1.push(x))
    }

    fn pop(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}
