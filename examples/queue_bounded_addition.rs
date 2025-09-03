use orx_meta::define_queue;

define_queue!(
    lifetimes => [];
    generics => [];
    elements => [Clone & Copy & Into<i64>];
    names => {
        traits: {
            queue: Queue,
            non_empty_queue: NonEmptyQueue
        },
        structs: {
            empty: Empty,
            single: Single,
            pair: Pair
        }
    };
);

// implement bounds on queue implementations

impl Into<i64> for Empty {
    fn into(self) -> i64 {
        0
    }
}

impl<F> Into<i64> for Single<F>
where
    F: Clone + Copy + Into<i64>,
{
    fn into(self) -> i64 {
        self.f.into()
    }
}

impl<F, B> Into<i64> for Pair<F, B>
where
    F: Clone + Copy + Into<i64>,
    B: Queue,
{
    fn into(self) -> i64 {
        self.f.into() + self.b.into()
    }
}

fn main() {
    #[derive(Clone, Copy, PartialEq, Debug)]
    struct MyFloat(f32);
    impl Into<i64> for MyFloat {
        fn into(self) -> i64 {
            self.0 as i64
        }
    }

    // grow

    let q = Empty::new();
    assert_eq!(0i64, q.into());

    let q = q.push_back(12i32);
    assert_eq!(12i64, q.into());

    let q = q.push_back(7u32);
    assert_eq!(19i64, q.into());

    let q = q.push_back(MyFloat(20.0));
    assert_eq!(39i64, q.into());

    // shrink

    let (f, q) = q.pop();
    assert_eq!(f, 12i32);
    assert_eq!(27i64, q.into());

    let (f, q) = q.pop();
    assert_eq!(f, 7u32);
    assert_eq!(20i64, q.into());

    let (f, q) = q.pop();
    assert_eq!(f, MyFloat(20.0));
    assert_eq!(0i64, q.into());
}
