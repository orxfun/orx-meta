use crate::Never;

pub trait Queue {
    type Push<X>: Queue;

    type Front;

    type Back: Queue;

    fn push<X>(self, x: X) -> Self::Push<X>;

    fn len(&self) -> usize;
}

pub trait NonEmptyQueue: Queue {
    fn pop_front(self) -> (Self::Front, Self::Back);
}

#[derive(Default, Debug)]
struct Empty;
impl Queue for Empty {
    type Push<X> = Single<X>;

    type Front = Never;

    type Back = Empty;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Single(x)
    }

    fn len(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Single<T>(T);
impl<T> Queue for Single<T> {
    type Push<X> = Pair<T, Single<X>>;

    type Front = T;

    type Back = Empty;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, Single(x))
    }

    fn len(&self) -> usize {
        1
    }
}
impl<T> NonEmptyQueue for Single<T> {
    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, Empty)
    }
}

#[derive(Debug)]
struct Pair<F, B>(F, B);
impl<F, B> Queue for Pair<F, B>
where
    B: Queue,
{
    type Push<X> = Pair<F, B::Push<X>>;

    type Front = F;

    type Back = B;

    fn push<X>(self, x: X) -> Self::Push<X> {
        Pair(self.0, self.1.push(x))
    }

    fn len(&self) -> usize {
        1 + self.1.len()
    }
}
impl<F, B> NonEmptyQueue for Pair<F, B>
where
    B: Queue,
{
    fn pop_front(self) -> (Self::Front, Self::Back) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let a = 0;

        let q = Empty;
        assert_eq!(q.len(), 0);

        let q = q.push(12);
        assert_eq!(q.len(), 1);
        dbg!(&q);

        let q = q.push('a');
        assert_eq!(q.len(), 2);
        dbg!(&q);

        let q = q.push(true);
        assert_eq!(q.len(), 3);
        dbg!(&q);

        let q = q.push(String::from("xyz"));
        assert_eq!(q.len(), 4);
        dbg!(&q);

        let (f, q) = q.pop_front();
        assert_eq!(q.len(), 3);
        dbg!(f);

        let (f, q) = q.pop_front();
        assert_eq!(q.len(), 2);
        dbg!(f);

        let (f, q) = q.pop_front();
        assert_eq!(q.len(), 1);
        dbg!(f);

        let (f, q) = q.pop_front();
        assert_eq!(q.len(), 0);
        dbg!(f);

        // assert!(a > 13);
    }
}
