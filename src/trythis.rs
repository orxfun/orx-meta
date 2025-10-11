#![allow(dead_code)]
use crate::define_queue;

// inputs

pub trait Input {}

define_queue!(
    lt => [];
    generics => [];
    elements => [];
    queue => [ InQ, InNeQ ; InEmpty, InSingle, InPair ];
    queue_of => in_of;
    builder => InBuilder;
);
impl Input for InEmpty {}
impl<F: Input> Input for InSingle<F> {}
impl<F: Input, B: InQ> Input for InPair<F, B> {}

// outputs

pub trait Output {}

define_queue!(
    lt => [];
    generics => [];
    elements => [];
    queue => [ OutQ, OutNeQ ; OutEmpty, OutSingle, OutPair ];
    queue_of => out_of;
    builder => OutBuilder;
);
impl Output for OutEmpty {}
impl<F: Output> Output for OutSingle<F> {}
impl<F: Output, B: OutQ> Output for OutPair<F, B> {}

// computation

pub trait Computation {
    type Input: Input;
    type Output: Output;
    fn run(&self, input: Self::Input) -> Self::Output;
}

// computation queue

pub trait Computations {
    type Input: InQ;
    type Output: OutQ;
    fn run(&self, input: Self::Input) -> Self::Output;
}
macro_rules! impl_computations_from_computation {
    ($comp:ty) => {
        impl Computations for $comp {
            type Input = InSingle<<Self as Computation>::Input>;
            type Output = OutSingle<<Self as Computation>::Output>;
            fn run(&self, input: Self::Input) -> Self::Output {
                OutSingle::new(<Self as Computation>::run(self, input.into_front()))
            }
        }
    };
}

define_queue!(
    lt => [];
    generics => [];
    elements => [Computations];
    queue => [ CompQ, CompNeQ ; CompEmpty, CompSingle, CompPair ];
    queue_of => out_of;
    builder => CompBuilder;
);
impl Computations for CompEmpty {
    type Input = InEmpty;
    type Output = OutEmpty;
    fn run(&self, _: Self::Input) -> Self::Output {
        Default::default()
    }
}
impl<F: Computations> Computations for CompSingle<F> {
    type Input = InSingle<F::Input>;
    type Output = OutSingle<F::Output>;
    fn run(&self, input: Self::Input) -> Self::Output {
        OutSingle::new(self.f.run(input.into_front()))
    }
}
impl<F: Computations, B: CompQ> Computations for CompPair<F, B> {
    type Input = InPair<F::Input, B::Input>;
    type Output = OutPair<F::Output, B::Output>;
    fn run(&self, input: Self::Input) -> Self::Output {
        let (in_f, in_b) = input.pop();
        let out_f = self.f.run(in_f);
        let out_b = self.b.run(in_b);
        OutPair::new(out_f, out_b)
    }
}

#[cfg(test)]
mod example {
    use super::*;

    // inputs
    impl Input for Vec<i32> {}
    impl Input for String {}

    // outputs
    impl Output for usize {}
    impl Output for Vec<i32> {}
    impl Output for Option<char> {}

    // independent computations
    struct AddNumToSeries(i32);
    impl Computation for AddNumToSeries {
        type Input = Vec<i32>;
        type Output = Vec<i32>;
        fn run(&self, mut input: Self::Input) -> Self::Output {
            input.iter_mut().for_each(|x| *x += self.0);
            input
        }
    }
    impl_computations_from_computation!(AddNumToSeries);

    struct LenOfString;
    impl Computation for LenOfString {
        type Input = String;
        type Output = usize;
        fn run(&self, input: Self::Input) -> Self::Output {
            input.len()
        }
    }
    impl_computations_from_computation!(LenOfString);

    struct FirstLetter;
    impl Computation for FirstLetter {
        type Input = String;
        type Output = Option<char>;
        fn run(&self, input: Self::Input) -> Self::Output {
            input.chars().next()
        }
    }
    impl_computations_from_computation!(FirstLetter);

    #[test]
    fn adhoc_computations() {
        let comp = CompEmpty::new()
            .push_back(AddNumToSeries(42))
            .push_back(LenOfString)
            .push_back(FirstLetter)
            .push_back(AddNumToSeries(33))
            .push_back(AddNumToSeries(12));

        let output = comp.run(
            InEmpty::new()
                .push_back(vec![1, 2, 3])
                .push_back("xyz".to_string())
                .push_back("hello".to_string())
                .push_back(vec![1, 2])
                .push_back(vec![1])
                .raise(),
        );

        dbg!(output);

        assert_eq!(comp.len(), 33);
    }
}
