#![allow(dead_code)]

// inputs

trait Input {}

trait InQueue
where
    Self: Input,
{
    type PushBack<X: Input>: InQueue;
    type Front: Input;
    type Back: InQueue;
    fn push_back<X: Input>(self, x: impl Into<InSingle<X>>) -> Self::PushBack<X>;

    type Raised: InQueue;
    fn raise(self) -> Self::Raised;
    fn from_raised(raised: Self::Raised) -> Self;
}

#[derive(Debug)]
struct InEmpty;
impl Input for InEmpty {}
impl InQueue for InEmpty {
    type PushBack<X: Input> = InSingle<X>;
    type Front = Self;
    type Back = Self;
    fn push_back<X: Input>(self, x: impl Into<InSingle<X>>) -> Self::PushBack<X> {
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
struct InSingle<F: Input>(F);
impl<F: Input> From<F> for InSingle<F> {
    fn from(value: F) -> Self {
        InSingle(value)
    }
}
impl<F: Input> Input for InSingle<F> {}
impl<F: Input> InQueue for InSingle<F> {
    type PushBack<X: Input> = InPair<F, InSingle<X>>;
    type Front = F;
    type Back = Self;
    fn push_back<X: Input>(self, x: impl Into<InSingle<X>>) -> Self::PushBack<X> {
        InPair(self.0, x.into())
    }

    type Raised = InSingle<InSingle<F>>;
    fn raise(self) -> Self::Raised {
        InSingle(self)
    }
    fn from_raised(raised: Self::Raised) -> Self {
        raised.0
    }
}

#[derive(Debug)]
struct InPair<F: Input, B: InQueue>(F, B);
impl<F: Input, B: InQueue> Input for InPair<F, B> {}
impl<F: Input, B: InQueue> InQueue for InPair<F, B> {
    type PushBack<X: Input> = InPair<F, B::PushBack<X>>;
    type Front = F;
    type Back = B;
    fn push_back<X: Input>(self, x: impl Into<InSingle<X>>) -> Self::PushBack<X> {
        InPair(self.0, self.1.push_back(x))
    }

    type Raised = InPair<InSingle<F>, B::Raised>;
    fn raise(self) -> Self::Raised {
        InPair(InSingle(self.0), self.1.raise())
    }
    fn from_raised(raised: Self::Raised) -> Self {
        let f = raised.0.0;
        let b = B::from_raised(raised.1);
        InPair(f, b)
    }
}

#[cfg(test)]
mod example {
    use super::*;

    // inputs
    impl Input for Vec<i32> {}
    impl Input for String {}

    // #[test]
    fn abc() {
        type X = InPair<Vec<i32>, InPair<String, InSingle<String>>>;

        let input = InEmpty
            .push_back(vec![1, 2, 3])
            .push_back("xyz".to_string())
            .push_back("hello".to_string());

        let raised = input.raise();

        dbg!(&raised);

        let input = X::from_raised(raised);

        dbg!(&input);

        assert_eq!(input.0.len(), 33);
    }
}
