use orx_meta::define_queue;
use std::fmt::{Debug, Display};

pub trait Analysis: Debug {
    type Input<'i>: Debug;

    type Output: Display;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output;
}

// some things to analyze

struct Hero {
    name: &'static str,
    attack: u32,
    defense: u32,
    power: u32,
    knowledge: u32,
}
impl Debug for Hero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}x{}x{}x{})",
            self.name, self.attack, self.defense, self.power, self.knowledge
        )
    }
}

struct Creature {
    name: &'static str,
    level: u32,
    hp: u32,
    is_range: bool,
}
impl Debug for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (lvl={},hp={},is-range={})",
            self.name, self.level, self.hp, self.is_range
        )
    }
}

// actual analysis

#[derive(Debug)]
struct AvgOfHeroAttack;
impl Analysis for AvgOfHeroAttack {
    type Input<'i> = &'i [Hero];
    type Output = u32;
    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        match input.len() {
            0 => 0,
            n => input.iter().map(|x| x.attack).sum::<u32>() / n as u32,
        }
    }
}

struct Rng {
    min: u32,
    max: u32,
}
impl Display for Rng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}
#[derive(Debug)]
struct CreatureHpRange;
impl Analysis for CreatureHpRange {
    type Input<'i> = &'i [Creature];
    type Output = Rng;
    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        let min = input.iter().map(|x| x.hp).min().unwrap_or(0);
        let max = input.iter().map(|x| x.hp).max().unwrap_or(0);
        Rng { min, max }
    }
}

#[derive(Debug)]
struct NumRangeCreatures;
impl Analysis for NumRangeCreatures {
    type Input<'i> = &'i [Creature];

    type Output = usize;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        input.iter().filter(|x| x.is_range).count()
    }
}

// # DEFINE COMPOSITION

// inputs
define_queue!(
    lifetimes => ['i];
    elements => [Debug];
    names => {
        traits: { queue: InQueue, non_empty_queue: NonEmptyInQueue },
        structs: { empty: InEmpty, single: InSingle, pair: InPair }
    };
);

// outputs
define_queue!(
    elements => [Display];
    names => {
        traits: { queue: OutQueue, non_empty_queue: NonEmptyOutQueue },
        structs: { empty: OutEmpty, single: OutSingle, pair: OutPair }
    };
);
impl Display for OutEmpty {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl<F: Display> Display for OutSingle<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.f)
    }
}
impl<F: Display, B: OutQueue> Display for OutPair<F, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} && {}", self.f, self.b)
    }
}

// define composition

pub trait Analyses: Debug {
    type Input<'i>: InQueue<'i>;

    type Output: OutQueue;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output;

    fn compose<'i, X: Analysis>(
        self,
        analysis: X,
    ) -> impl Analyses<
        Input<'i> = <Self::Input<'i> as InQueue<'i>>::PushBack<X::Input<'i>>,
        Output = <Self::Output as OutQueue>::PushBack<X::Output>,
    >;
}

#[derive(Debug)]
struct AnalysesEmpty;
impl Analyses for AnalysesEmpty {
    type Input<'i> = InEmpty<'i>;

    type Output = OutEmpty;

    fn analyze(&self, _: &Self::Input<'_>) -> Self::Output {
        OutEmpty::new()
    }

    fn compose<'i, X: Analysis>(
        self,
        analysis: X,
    ) -> impl Analyses<
        Input<'i> = <Self::Input<'i> as InQueue<'i>>::PushBack<X::Input<'i>>,
        Output = <Self::Output as OutQueue>::PushBack<X::Output>,
    > {
        AnalysesSingle(analysis)
    }
}

#[derive(Debug)]
struct AnalysesSingle<F: Analysis>(F);
impl<F: Analysis> Analyses for AnalysesSingle<F> {
    type Input<'i> = InSingle<'i, F::Input<'i>>;

    type Output = OutSingle<F::Output>;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        OutSingle::new(self.0.analyze(input.front()))
    }

    fn compose<'i, X: Analysis>(
        self,
        analysis: X,
    ) -> impl Analyses<
        Input<'i> = <Self::Input<'i> as InQueue<'i>>::PushBack<X::Input<'i>>,
        Output = <Self::Output as OutQueue>::PushBack<X::Output>,
    > {
        AnalysesPair(self.0, AnalysesSingle(analysis))
    }
}

#[derive(Debug)]
struct AnalysesPair<F: Analysis, B: Analyses>(F, B);
impl<F: Analysis, B: Analyses> Analyses for AnalysesPair<F, B> {
    type Input<'i> = InPair<'i, F::Input<'i>, B::Input<'i>>;

    type Output = OutPair<F::Output, B::Output>;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        let (in0, in1) = input.front_back();
        let out0 = self.0.analyze(in0);
        let out1 = self.1.analyze(in1);
        OutPair::new(out0, out1)
    }

    fn compose<'i, X: Analysis>(
        self,
        analysis: X,
    ) -> impl Analyses<
        Input<'i> = <Self::Input<'i> as InQueue<'i>>::PushBack<X::Input<'i>>,
        Output = <Self::Output as OutQueue>::PushBack<X::Output>,
    > {
        AnalysesPair(self.0, self.1.compose(analysis))
    }
}

fn main() {
    // inputs
    let heroes = vec![
        Hero {
            name: "Solymr",
            attack: 0,
            defense: 0,
            power: 2,
            knowledge: 3,
        },
        Hero {
            name: "Crag Hack",
            attack: 4,
            defense: 0,
            power: 1,
            knowledge: 1,
        },
    ];

    let creatures = vec![
        Creature {
            name: "Titan",
            level: 7,
            hp: 300,
            is_range: true,
        },
        Creature {
            name: "Thunderbird",
            level: 5,
            hp: 60,
            is_range: false,
        },
        Creature {
            name: "Ancient Behemoth",
            level: 7,
            hp: 300,
            is_range: false,
        },
    ];

    // empty analysis

    let analyses = AnalysesEmpty;
    let input = InEmpty::new();
    let output = analyses.analyze(&input);
    println!(
        "\n# Empty analysis\n  analyses => {:?}\n  inputs => {:?}\n  outputs => {}",
        analyses, input, output
    );

    // single analyses
    let analyses = AnalysesEmpty.compose(AvgOfHeroAttack);
    let input = InEmpty::new().push_back(heroes.as_slice());
    let output = analyses.analyze(&input);
    println!(
        "\n# Single analysis\n  analyses => {:?}\n  inputs => {:?}\n  outputs => {}",
        analyses, input, output
    );

    // pair of analyses
    let analyses = AnalysesEmpty
        .compose(AvgOfHeroAttack)
        .compose(CreatureHpRange);
    let input = InEmpty::new()
        .push_back(heroes.as_slice())
        .push_back(creatures.as_slice());
    let output = analyses.analyze(&input);
    println!(
        "\n# Single analysis\n  analyses => {:?}\n  inputs => {:?}\n  outputs => {}",
        analyses, input, output
    );
}
