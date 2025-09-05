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
    fn push_back<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X>;

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
    fn push_back<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
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
    fn push_back<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
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

#[derive(Debug)]
struct P<F: Input, B: InQueue>(F, B);
impl<F: Input, B: InQueue> Input for P<F, B> {}
impl<F: Input, B: InQueue> InQueue for P<F, B> {
    type PushBack<X: Input> = P<F, B::PushBack<X>>;
    type Front = F;
    type Back = B;
    fn push_back<X: Input>(self, x: impl Into<S<X>>) -> Self::PushBack<X> {
        P(self.0, self.1.push_back(x))
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

#[cfg(test)]
mod example {
    use super::*;

    // inputs
    impl Input for Vec<i32> {}
    impl Input for String {}
    impl Input for bool {}
    impl Input for char {}

    #[test]
    fn abc() {
        type X = P<Vec<i32>, P<String, S<String>>>;

        let input = InEmpty
            .push_back(vec![1, 2, 3])
            .push_back("xyz".to_string())
            .push_back("hello".to_string())
            .push_back(true)
            .push_back('x');

        let raised = input.raise();

        dbg!(&raised);

        // let input = X::from_raised(raised);

        // dbg!(&input);

        assert_eq!(raised.0.0.len(), 33);
    }
}
