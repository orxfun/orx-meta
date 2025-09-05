use core::fmt::Debug;
use orx_meta::{define_queue_core_zzz, define_queue_tuple_transformation_zzz};

pub trait Analysis: Debug + Default {
    type Input<'i>: Debug + Print;

    type Output: Clone + Debug + Print;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output;
}

pub trait Print {
    fn print(&self);
}
impl<T: Debug> Print for &[T] {
    fn print(&self) {
        let mut iter = self.iter();
        print!("[");
        if let Some(x) = iter.next() {
            print!("{x:?}");
            for x in iter {
                print!(", {x:?}")
            }
        }
        print!("]");
    }
}

// some things to analyze

struct Hero {
    name: &'static str,
    attack: u32,
    knowledge: u32,
}
impl Debug for Hero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}x{})", self.name, self.attack, self.knowledge)
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

#[derive(Debug, Default)]
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
impl Print for u32 {
    fn print(&self) {
        print!("{self}");
    }
}

#[derive(Clone, Debug)]
struct Rng {
    min: u32,
    max: u32,
}
#[derive(Debug, Default)]
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
impl Print for Rng {
    fn print(&self) {
        print!("[{}, {}]", self.min, self.max);
    }
}

#[derive(Debug, Default)]
struct NumRangeCreatures;
impl Analysis for NumRangeCreatures {
    type Input<'i> = &'i [Creature];

    type Output = usize;

    fn analyze(&self, input: &Self::Input<'_>) -> Self::Output {
        input.iter().filter(|x| x.is_range).count()
    }
}
impl Print for usize {
    fn print(&self) {
        print!("{self}");
    }
}

// # DEFINE COMPOSITION

// inputs
define_queue_core_zzz!(
    lifetimes => ['i];
    elements => [Debug & Print];
    names => {
        traits: { queue: InQueue, non_empty_queue: NonEmptyInQueue },
        structs: { empty: InEmpty, single: InSingle, pair: InPair }
    };
);
impl Print for InEmpty<'_> {
    fn print(&self) {}
}
impl<F: Debug + Print> Print for InSingle<'_, F> {
    fn print(&self) {
        print!("=i=> ");
        self.f.print();
    }
}
impl<'i, F: Debug + Print, B: InQueue<'i>> Print for InPair<'i, F, B> {
    fn print(&self) {
        print!("=i=> ");
        self.f.print();
        println!();
        self.b.print();
    }
}

// outputs
define_queue_core_zzz!(
    elements => [Clone & Debug & Print];
    names => {
        traits: { queue: OutQueue, non_empty_queue: NonEmptyOutQueue },
        structs: { empty: OutEmpty, single: OutSingle, pair: OutPair }
    };
);
impl Print for OutEmpty {
    fn print(&self) {}
}
impl<F: Clone + Debug + Print> Print for OutSingle<F> {
    fn print(&self) {
        print!("=o=> ");
        self.f.print();
    }
}
impl<F: Clone + Debug + Print, B: OutQueue> Print for OutPair<F, B> {
    fn print(&self) {
        print!("=o=> ");
        self.f.print();
        println!();
        self.b.print();
    }
}

// define composition

pub trait Analyses: Debug + Print + Default {
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

#[derive(Debug, Default)]
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

impl Print for AnalysesEmpty {
    fn print(&self) {
        print!("AnalysesEmpty")
    }
}

#[derive(Debug, Default)]
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
impl<F: Analysis> Print for AnalysesSingle<F> {
    fn print(&self) {
        print!("{:?}", self.0);
    }
}

#[derive(Debug, Default)]
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
impl<F: Analysis, B: Analyses> Print for AnalysesPair<F, B> {
    fn print(&self) {
        print!("{:?} & ", self.0);
        self.1.print();
    }
}

fn print_results<'i, A: Analyses>(analyses: &A, inputs: &A::Input<'i>, outputs: &A::Output) {
    print!("\n# ");
    analyses.print();
    println!();

    println!("Inputs");
    inputs.print();
    println!();

    println!("Outputs");
    outputs.print();
    println!();
}

fn get_input(id: usize) -> (Vec<Hero>, Vec<Creature>) {
    match id % 2 == 0 {
        true => {
            let heroes = vec![
                Hero {
                    name: "Solymr",
                    attack: 0,
                    knowledge: 3,
                },
                Hero {
                    name: "Crag Hack",
                    attack: 4,
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
            (heroes, creatures)
        }
        false => {
            let heroes = vec![
                Hero {
                    name: "Kyrre",
                    attack: 1,
                    knowledge: 1,
                },
                Hero {
                    name: "Vidomina",
                    attack: 1,
                    knowledge: 2,
                },
            ];

            let creatures = vec![
                Creature {
                    name: "Archangel",
                    level: 7,
                    hp: 250,
                    is_range: false,
                },
                Creature {
                    name: "Grand Elf",
                    level: 3,
                    hp: 15,
                    is_range: true,
                },
                Creature {
                    name: "Skeleton Warrior",
                    level: 1,
                    hp: 6,
                    is_range: false,
                },
            ];
            (heroes, creatures)
        }
    }
}

fn adhoc_analyses() {
    let (heroes, creatures) = get_input(0);

    // empty analysis

    let analyses = AnalysesEmpty;
    let input = InEmpty::new();
    let output = analyses.analyze(&input);
    print_results(&analyses, &input, &output);

    // single analyses
    let analyses = AnalysesEmpty.compose(AvgOfHeroAttack);
    let input = InEmpty::new().push_back(heroes.as_slice());
    let output = analyses.analyze(&input);
    print_results(&analyses, &input, &output);

    // three analysis
    let analyses = AnalysesEmpty
        .compose(AvgOfHeroAttack)
        .compose(CreatureHpRange)
        .compose(NumRangeCreatures);
    let input = InEmpty::new()
        .push_back(heroes.as_slice())
        .push_back(creatures.as_slice())
        .push_back(creatures.as_slice());
    let output = analyses.analyze(&input);
    print_results(&analyses, &input, &output);
}

define_queue_tuple_transformation_zzz!(
    lifetimes => ['i];
    elements => [Debug & Print];
    queues => { trait: InQueue, empty: InEmpty, single: InSingle, pair: InPair };
);
define_queue_tuple_transformation_zzz!(
    elements => [Clone & Debug & Print];
    queues => { trait: OutQueue, empty: OutEmpty, single: OutSingle, pair: OutPair };
);

fn defined_analyses_1() {
    type Analyses2<A, B> = AnalysesPair<A, AnalysesSingle<B>>;
    type MyAnalyses = Analyses2<CreatureHpRange, NumRangeCreatures>;

    fn run(input: &<MyAnalyses as Analyses>::Input<'_>) -> <MyAnalyses as Analyses>::Output {
        MyAnalyses::default().analyze(input)
    }

    let (_, c) = get_input(0);
    let input = (c.as_slice(), c.as_slice()).into();
    let outputs = run(&input).into_tuple();
    println!("{outputs:?}");

    let (_, c) = get_input(1);
    let input = (c.as_slice(), c.as_slice()).into();
    let outputs = run(&input).into_tuple();
    println!("{outputs:?}");
}

fn defined_analyses_2() {
    type Analyses3<A, B, C> = AnalysesPair<A, AnalysesPair<B, AnalysesSingle<C>>>;
    type MyAnalyses = Analyses3<AvgOfHeroAttack, CreatureHpRange, NumRangeCreatures>;

    fn run(input: &<MyAnalyses as Analyses>::Input<'_>) -> <MyAnalyses as Analyses>::Output {
        MyAnalyses::default().analyze(input)
    }

    let (h, c) = get_input(0);
    let input = (h.as_slice(), c.as_slice(), c.as_slice()).into();
    let outputs = run(&input).into_tuple();
    println!("{outputs:?}");

    let (h, c) = get_input(1);
    let input = (h.as_slice(), c.as_slice(), c.as_slice()).into();
    let outputs = run(&input).into_tuple();
    println!("{outputs:?}");
}

fn main() {
    println!("\n\n ---  ADHOC ANALYSES  --- ");
    adhoc_analyses();

    println!("\n\n ---  DEFINED ANALYSES - 1 --- ");
    defined_analyses_1();

    println!("\n\n ---  DEFINED ANALYSES - 2 --- ");
    defined_analyses_2();
}
