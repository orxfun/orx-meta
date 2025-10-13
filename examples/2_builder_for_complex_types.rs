//! Notice that any `struct` can be considered as a tuple of its fields.
//! Since any tuple can be represented as a queue, so can any struct.
//!
//! Then, we can use the `QueueBuilder` as a generic builder for any
//! complex struct.

use orx_meta::queue::*;
use orx_meta::queue_of;

fn main() {
    // # 1. build an instance of a queue with a specific type

    {
        // this is our target type that we need instances of
        type MyQueue = queue_of!(u32, bool, char, String);

        // notice that `push` can only be called with the correct type
        // and finish is available only when all elements are pushed
        let val: MyQueue = QueueBuilder::<MyQueue>::new()
            .push(42)
            .push(true)
            .push('x')
            .push(String::from("foo"))
            .finish();
        assert_eq!(val.as_tuple(), (&42, &true, &'x', &String::from("foo")));
    }

    // # 2. use it as a generic builder for any tuple

    {
        // since any tuple can be represented as a queue;
        // we can use QueueBuilder to build any tuple

        type MyQueue = queue_of!(u32, bool, char, String);

        let val: (u32, bool, char, String) = QueueBuilder::<MyQueue>::new()
            .push(42)
            .push(true)
            .push('x')
            .push(String::from("foo"))
            .finish()
            .into_tuple();
        assert_eq!(val, (42, true, 'x', String::from("foo")));
    }

    // # 3. use it as a generic builder of structs that implements From tuple

    {
        #[derive(PartialEq, Eq, Debug)]
        struct MyComplexStruct {
            a: u32,
            b: bool,
            c: char,
            d: String,
        }
        impl From<(u32, bool, char, String)> for MyComplexStruct {
            fn from((a, b, c, d): (u32, bool, char, String)) -> Self {
                Self { a, b, c, d }
            }
        }

        type MyQueue = queue_of!(u32, bool, char, String);
        let val: MyComplexStruct = QueueBuilder::<MyQueue>::new()
            .push(42)
            .push(true)
            .push('x')
            .push(String::from("foo"))
            .finish()
            .into_tuple()
            .into();

        assert_eq!(
            val,
            MyComplexStruct {
                a: 42,
                b: true,
                c: 'x',
                d: String::from("foo")
            }
        );
    }

    // # 3-b. use it as a generic builder of structs that implements From queue

    {
        #[derive(PartialEq, Eq, Debug)]
        struct MyComplexStruct {
            a: u32,
            b: bool,
            c: char,
            d: String,
        }
        impl From<queue_of!(u32, bool, char, String)> for MyComplexStruct {
            fn from(queue: queue_of!(u32, bool, char, String)) -> Self {
                let (a, b, c, d) = queue.into_tuple();
                Self { a, b, c, d }
            }
        }

        type MyQueue = queue_of!(u32, bool, char, String);
        let val: MyComplexStruct = QueueBuilder::<MyQueue>::new()
            .push(42)
            .push(true)
            .push('x')
            .push(String::from("foo"))
            .finish()
            .into();

        assert_eq!(
            val,
            MyComplexStruct {
                a: 42,
                b: true,
                c: 'x',
                d: String::from("foo")
            }
        );
    }
}
