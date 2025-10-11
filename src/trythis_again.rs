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
