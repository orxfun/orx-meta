use crate::product::{empty::Empty, pop::Pop, push::Push, single::Single};

pub struct Multi<F, M, B>
where
    M: Push,
{
    front: F,
    middle: M,
    back: B,
}

impl<F, M, B> From<(F, M, B)> for Multi<F, M, B>
where
    M: Push,
{
    fn from((front, middle, back): (F, M, B)) -> Self {
        Self {
            front,
            middle,
            back,
        }
    }
}

impl<F, M, B> Push for Multi<F, M, B>
where
    M: Push,
{
    type PushBack<X> = Multi<F, M::PushBack<B>, X>;

    type PushFront<X> = Multi<X, M::PushFront<F>, B>;

    const LEN: usize = 2 + M::LEN;

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        (self.front, self.middle.push_back(self.back), element).into()
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        (element, self.middle.push_front(self.front), self.back).into()
    }
}

impl<F, M, B> Pop for Multi<F, M, B>
where
    M: Push,
{
    type Front = F;

    type PopFront = M::PushBack<B>;

    type Back = B;

    type PopBack = M::PushFront<F>;

    fn pop_back(self) -> (Self::Back, Self::PopBack) {
        (self.back, self.middle.push_front(self.front))
    }

    fn pop_front(self) -> (Self::Front, Self::PopFront) {
        (self.front, self.middle.push_back(self.back))
    }
}

impl<F, M, B> Multi<F, M, B>
where
    M: Push,
{
    // ref

    pub fn back(&self) -> &B {
        &self.back
    }

    pub fn front(&self) -> &F {
        &self.front
    }

    pub fn middle(&self) -> &M {
        &self.middle
    }

    // mut

    fn back_mut(&mut self) -> &mut B {
        &mut self.back
    }

    fn front_mut(&mut self) -> &mut F {
        &mut self.front
    }
}

impl<F, B> Multi<F, Empty, B> {
    pub fn into_tuple(self) -> (F, B) {
        (self.front, self.back)
    }

    pub fn as_tuple(&self) -> (&F, &B) {
        (&self.front, &self.back)
    }
}

impl<F, B, X3> Multi<F, Single<X3>, B> {
    pub fn into_tuple(self) -> (F, X3, B) {
        (self.front, self.middle.pop_back().0, self.back)
    }

    pub fn as_tuple(&self) -> (&F, &X3, &B) {
        (&self.front, self.middle.front(), &self.back)
    }
}

impl<F, B, X3, X4> Multi<F, Multi<X3, Empty, X4>, B> {
    pub fn into_tuple(self) -> (F, X3, X4, B) {
        let (x3, x4) = self.middle.pop_front();
        let x4 = x4.pop_back().0;
        (self.front, x3, x4, self.back)
    }

    pub fn as_tuple(&self) -> (&F, &X3, &X4, &B) {
        (
            &self.front,
            self.middle.front(),
            self.middle.back(),
            &self.back,
        )
    }
}

impl<F, B, X3, X4, X5> Multi<F, Multi<X3, Single<X5>, X4>, B> {
    pub fn into_tuple(self) -> (F, X3, X4, B) {
        let (x3, x4) = self.middle.pop_front();
        let x4 = x4.pop_back().0;
        (self.front, x3, x4, self.back)
    }

    pub fn as_tuple(&self) -> (&F, &X3, &X4, &B) {
        (
            &self.front,
            self.middle.front(),
            self.middle.back(),
            &self.back,
        )
    }
}
