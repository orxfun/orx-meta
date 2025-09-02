use orx_meta::define_queue;

define_queue!(
    lifetimes => [];
    generics => [];
    elements => [];
    names => {
        traits: {
            queue: Queue,
            non_empty_queue: NonEmptyQueue,
        },
        structs: {
            empty: Empty,
            single: Single,
            pair: Pair,
            composition: QueueComposition,
            builder: Builder,
        },
    };
);

fn as_queue_of_different_type_elements() {
    let q = Empty::new().push_back('x').push_back(12).push_back(true);

    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);

    let (f, q) = q.pop();
    assert_eq!(f, 'x');
    assert_eq!(q.front(), &12);
    assert_eq!(q.len(), 2);

    let (f, q) = q.pop();
    assert_eq!(f, 12);
    assert_eq!(q.front(), &true);
    assert_eq!(q.len(), 1);

    let (f, q) = q.pop();
    assert_eq!(f, true);
    assert!(q.is_empty());
}

fn basic_builder_pattern() {
    type Input = Pair<char, Pair<i32, Pair<bool, Single<String>>>>;

    fn concat(input: Input) -> String {
        let (a, b, c, d) = input.into_tuple();
        format!("{}-{}-{}-{}", a, b, c, d)
    }

    let input = Builder::<Input, _>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true)
        .push_back("y".to_string())
        .finish();

    let result = concat(input);
    assert_eq!(result, String::from("x-12-true-y"));
}

fn main() {
    as_queue_of_different_type_elements();
    basic_builder_pattern();
}
