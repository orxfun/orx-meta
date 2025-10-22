//! you may find the article [here](https://github.com/orxfun/orx-meta/blob/main/docs/2_generic_builder.md)

#[allow(unused_variables, dead_code)]

fn main() {
    use orx_meta::queue::*;
    use orx_meta::queue_of;

    // # 1. Type Alias Helper

    type Q = Queue<i32, Queue<bool, Queue<char, QueueSingle<String>>>>;
    let q: Q = Queue::new(42).push(true).push('x').push("foo".to_string());

    type R = queue_of!(i32, bool, char, String); // R == Q
    let r: R = q;

    // # 2. Type-safe Generic Builder for any Queue

    // this is our target type that we need instances of
    type MyQueue = queue_of!(u32, bool, char, String);

    // notice that `push` can only be called with the correct type
    // and finish is available only when all elements are pushed
    let val: MyQueue = QueueBuilder::<MyQueue>::new()
        .push(42)
        .push(true)
        .push('x')
        .push("foo".to_string())
        .finish();
    assert_eq!(val.as_tuple(), (&42, &true, &'x', &"foo".to_string()));

    // # 3. use it as a generic builder of structs that implements From queue

    #[derive(PartialEq, Eq, Debug)]
    struct ComplexStruct {
        a: u32,
        b: bool,
        c: char,
        d: String,
    }
    impl From<queue_of!(u32, bool, char, String)> for ComplexStruct {
        fn from(queue: queue_of!(u32, bool, char, String)) -> Self {
            let (a, b, c, d) = queue.into_tuple();
            Self { a, b, c, d }
        }
    }

    let val: ComplexStruct = QueueBuilder::<queue_of!(u32, bool, char, String)>::new()
        .push(42)
        .push(true)
        .push('x')
        .push("foo".to_string())
        .finish()
        .into();

    assert_eq!(
        val,
        ComplexStruct {
            a: 42,
            b: true,
            c: 'x',
            d: "foo".to_string()
        }
    );
}
