// use crate::define_non_empty_queue;

// // bounds

// // each item must be of Sth<'i> and Clone; we reduce them to marker trait Req<'i> (one trait & one lifetime)

// pub trait Sth<'i> {} // actual requirement

// pub trait Req<'i> {} // marker requirement that combines Sth<'i> and Clone

// impl<'i, X> Req<'i> for X where X: Sth<'i> + Clone {}

// define_non_empty_queue!(Queue, MultiQueue, Single, Pair, QueueComposition, Req, 'a);

// // tests

// impl<'i> Req<'i> for char {}
// impl<'i> Req<'i> for i32 {}
// impl<'i> Req<'i> for String {}
// impl<'i> Req<'i> for bool {}

// #[test]
// fn one() {
//     let x = Single('x', core::marker::PhantomData);

//     assert_eq!(x.front(), &'x');
//     let f = x.into_front();
//     assert_eq!(f, 'x');
// }

// #[test]
// fn two() {
//     let x = Single('x', core::marker::PhantomData);
//     let x = x.push_back(32);

//     assert_eq!(x.front(), &'x');
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 'x');

//     assert_eq!(x.front(), &32);
//     let f = x.into_front();
//     assert_eq!(f, 32);
// }

// #[test]
// fn three() {
//     let x = Single('x', core::marker::PhantomData);
//     let x = x.push_back(32);
//     let x = x.push_back(String::from("xyz"));

//     assert_eq!(x.front(), &'x');
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 'x');

//     assert_eq!(x.front(), &32);
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 32);

//     assert_eq!(x.front(), &String::from("xyz"));
//     let f = x.into_front();
//     assert_eq!(f, String::from("xyz"));
// }

// #[test]
// fn four() {
//     let x = Single('x', core::marker::PhantomData);
//     let x = x.push_back(32);
//     let x = x.push_back(String::from("xyz"));
//     let x = x.push_back(true);

//     assert_eq!(x.front(), &'x');
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 'x');

//     assert_eq!(x.front(), &32);
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 32);

//     assert_eq!(x.front(), &String::from("xyz"));
//     let (f, x) = x.pop_front();
//     assert_eq!(f, String::from("xyz"));

//     assert_eq!(x.front(), &true);
//     let f = x.into_front();
//     assert_eq!(f, true);
// }

// #[test]
// fn compose_four() {
//     type C = QueueComposition;

//     let x = C::single('x');
//     let x = C::compose(x, 32);
//     let x = C::compose(x, String::from("xyz"));
//     let x = C::compose(x, true);

//     assert_eq!(x.len(), 4);

//     assert_eq!(x.front(), &'x');
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 'x');

//     assert_eq!(x.front(), &32);
//     let (f, x) = x.pop_front();
//     assert_eq!(f, 32);

//     let (f, b) = x.front_back();
//     assert_eq!(f, &String::from("xyz"));
//     assert_eq!(b.len(), 1);
//     assert_eq!(b.front(), &true);

//     assert_eq!(x.front(), &String::from("xyz"));
//     let (f, x) = x.pop_front();
//     assert_eq!(f, String::from("xyz"));

//     assert_eq!(x.front(), &true);
//     let f = x.into_front();
//     assert_eq!(f, true);
// }
