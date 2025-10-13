// use orx_meta::define_queue_core_zzz;

// define_queue_core_zzz!(
//     elements => [Into<i64>];
//     names => { traits: { queue: AdditiveQueue, non_empty_queue: NonEmptyAdditiveQueue },
//         structs: { empty: NoNum, single: OneNum, pair: MultiNumbers }
//     };
// );

// // implement bounds on queue implementations

// impl Into<i64> for NoNum {
//     fn into(self) -> i64 {
//         0 // addition identity
//     }
// }

// impl<F> Into<i64> for OneNum<F>
// where
//     F: Into<i64>,
// {
//     fn into(self) -> i64 {
//         // implementation of single is usually pretty standard
//         self.f.into()
//     }
// }

// impl<F, B> Into<i64> for MultiNumbers<F, B>
// where
//     F: Into<i64>,
//     B: AdditiveQueue,
// {
//     fn into(self) -> i64 {
//         // define composition
//         // notice that self.b.into() is recursive since B itself is a queue
//         self.f.into() + self.b.into()
//     }
// }

// fn main() {
//     #[derive(Clone, Copy, PartialEq, Debug)]
//     struct MyFloat(f32);
//     impl Into<i64> for MyFloat {
//         fn into(self) -> i64 {
//             self.0 as i64
//         }
//     }

//     // now we can have an additive queue containing different types of numbers
//     // as long as they can all be converted into i64

//     // we can then convert the additive queue into i64, result is the sum of its elements

//     // grow

//     let q = NoNum::new();
//     assert_eq!(0i64, q.into());

//     let q = q.push(12i32);
//     assert_eq!(12i64, q.into());

//     let q = q.push(7u32);
//     assert_eq!(12i64 + 7, q.into());

//     let q = q.push(MyFloat(20.0));
//     assert_eq!(12i64 + 7 + 20, q.into());

//     // shrink

//     let (f, q) = q.pop();
//     assert_eq!(f, 12i32);
//     assert_eq!(7i64 + 20, q.into());

//     let (f, q) = q.pop();
//     assert_eq!(f, 7u32);
//     assert_eq!(20i64, q.into());

//     let (f, q) = q.pop();
//     assert_eq!(f, MyFloat(20.0));
//     assert_eq!(0i64, q.into());

//     // chain

//     let sum: i64 = NoNum::new()
//         .push(12i32)
//         .push(7u32)
//         .push(MyFloat(20.0))
//         .into();
//     assert_eq!(sum, 12 + 7 + 20);
// }

fn main() {}
