#![allow(dead_code)]

// inputs

use crate::trythis::InQ;

trait Input {}

trait InQueue
where
    Self: Input,
{
    type PushBack<X: Input>: InQueue;
    type Front: Input;
    type Back: InQueue;
    fn push<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X>;

    type Raised: InQueue;
    fn raise(self) -> Self::Raised;
    fn from_raised(raised: Self::Raised) -> Self;
}

#[derive(Debug)]
struct InEmpty;
impl Input for InEmpty {}
impl InQueue for InEmpty {
    type PushBack<X: Input> = S<X>;
    type Front = Self;
    type Back = Self;
    fn push<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
        x.into()
    }

    type Raised = Self;
    fn raise(self) -> Self::Raised {
        self
    }
    fn from_raised(raised: Self::Raised) -> Self {
        raised
    }
}

#[derive(Debug)]
struct S<F: Input>(F);
impl<F: Input> From<F> for S<F> {
    fn from(value: F) -> Self {
        S(value)
    }
}
impl<F: Input> Input for S<F> {}
impl<F: Input> InQueue for S<F> {
    type PushBack<X: Input> = P<F, S<X>>;
    type Front = F;
    type Back = Self;
    fn push<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
        P(self.0, x.into())
    }

    type Raised = S<Self>;
    fn raise(self) -> Self::Raised {
        S(self)
    }
    fn from_raised(raised: Self::Raised) -> Self {
        raised.0
    }
}
impl<F: Input> S<S<F>> {
    fn lower(self) -> S<F> {
        self.0
    }
}

#[derive(Debug)]
struct P<F: Input, B: InQueue>(F, B);
impl<F: Input, B: InQueue> Input for P<F, B> {}
impl<F: Input, B: InQueue> InQueue for P<F, B> {
    type PushBack<X: Input> = P<F, B::PushBack<X>>;
    type Front = F;
    type Back = B;
    fn push<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
        P(self.0, self.1.push(x))
    }

    type Raised = P<S<F>, B::Raised>;
    fn raise(self) -> Self::Raised {
        P(S(self.0), self.1.raise())
    }
    fn from_raised(raised: Self::Raised) -> Self {
        let f = raised.0.0;
        let b = B::from_raised(raised.1);
        P(f, b)
    }
}

impl<F: Input, B: InQueue> P<S<F>, B> {
    fn lower(self) -> S<F> {
        // self.0
        todo!()
    }
}
