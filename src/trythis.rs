#![allow(dead_code)]
use crate::define_queue;

// inputs

pub trait Input {}

define_queue!(
    elements => [Input];
    names => {
        traits: { queue: InQueue, non_empty_queue: InNonEmptyQueue },
        structs: { empty: InEmpty, single: InSingle, pair: InPair }
    };
);
impl Input for InEmpty {}
impl<F: Input> Input for InSingle<F> {}
impl<F: Input, B: InQueue> Input for InPair<F, B> {}

macro_rules! queue_of {
    () => {
        InEmpty
    };

    ($t1:ty) => {
        InSingle<$t1>
    };

    ($t1:ty, $t2:ty) => {
        InPair<$t1, InSingle<$t2>>
    };

    ($t1:ty, $t2:ty, $t3:ty) => {
        InPair<$t1, InPair<$t2, InSingle<$t3>>>
    };
}

// outputs

pub trait Output {}

define_queue!(
    elements => [Output];
    names => {
        traits: { queue: OutQueue, non_empty_queue: OutNonEmptyQueue },
        structs: { empty: OutEmpty, single: OutSingle, pair: OutPair }
    };
);
impl Output for OutEmpty {}
impl<F: Output> Output for OutSingle<F> {}
impl<F: Output, B: OutQueue> Output for OutPair<F, B> {}

// computation

pub trait Computation {
    type Input: Input;
    type Output: Output;
    fn run(&self, input: Self::Input) -> Self::Output;
}

// computation queue

pub trait Computations {
    type Input: InQueue;
    type Output: OutQueue;
    fn run(&self, input: Self::Input) -> Self::Output;
}

define_queue!(
    elements => [Computations];
    names => {
        traits: { queue: CompQueue, non_empty_queue: CompNonEmptyQueue },
        structs: { empty: CompEmpty, single: CompSingle, pair: CompPair }
    };
);

impl Computations for CompEmpty {
    type Input = InEmpty;
    type Output = OutEmpty;
    fn run(&self, _: Self::Input) -> Self::Output {
        OutEmpty::new()
    }
}

impl<F: Computations> Computations for CompSingle<F> {
    type Input = InSingle<F::Input>;
    type Output = OutSingle<F::Output>;
    fn run(&self, input: Self::Input) -> Self::Output {
        OutSingle::new(self.f.run(input.into_front()))
    }
}

impl<F: Computations, B: CompQueue> Computations for CompPair<F, B> {
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
    impl Computations for AddNumToSeries {
        type Input = InSingle<<Self as Computation>::Input>;
        type Output = OutSingle<<Self as Computation>::Output>;
        fn run(&self, input: Self::Input) -> Self::Output {
            OutSingle::new(<Self as Computation>::run(self, input.into_front()))
        }
    }

    struct LenOfString;
    impl Computation for LenOfString {
        type Input = String;
        type Output = usize;
        fn run(&self, input: Self::Input) -> Self::Output {
            input.len()
        }
    }
    impl Computations for LenOfString {
        type Input = InSingle<<Self as Computation>::Input>;
        type Output = OutSingle<<Self as Computation>::Output>;
        fn run(&self, input: Self::Input) -> Self::Output {
            OutSingle::new(<Self as Computation>::run(self, input.into_front()))
        }
    }

    struct FirstLetter;
    impl Computation for FirstLetter {
        type Input = String;
        type Output = Option<char>;
        fn run(&self, input: Self::Input) -> Self::Output {
            input.chars().next()
        }
    }
    impl Computations for FirstLetter {
        type Input = InSingle<<Self as Computation>::Input>;
        type Output = OutSingle<<Self as Computation>::Output>;
        fn run(&self, input: Self::Input) -> Self::Output {
            OutSingle::new(<Self as Computation>::run(self, input.into_front()))
        }
    }

    #[test]
    fn adhoc_computations() {
        let comp = CompEmpty::new()
            .push_back(AddNumToSeries(42))
            .push_back(LenOfString)
            .push_back(FirstLetter);

        let input = InEmpty::new()
            .push_back(vec![1, 2, 3])
            .push_back("xyz".to_string())
            .push_back("hello".to_string());

        let output = comp.run(
            InEmpty::new()
                .push_back(InSingle::from(vec![1, 2, 3]))
                .push_back(InSingle::from("xyz".to_string()))
                .push_back(InSingle::from("hello".to_string())),
        );

        dbg!(output);

        assert_eq!(comp.len(), 33);
    }
}
