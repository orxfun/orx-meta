use crate::product::{pop::Pop, push::Push};

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

    pub fn middle_mut(&mut self) -> &mut M {
        &mut self.middle
    }
}
