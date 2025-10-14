// #[allow(unused_variables, dead_code)]
// fn main() {
//     // # Dynamic collection of Anything
//     use std::any::Any;

//     let mut any_vec: Vec<Box<dyn Any>> = vec![];
//     any_vec.push(Box::new(42));
//     any_vec.push(Box::new(true));
//     any_vec.push(Box::new('x'));
//     any_vec.push(Box::new("foo"));

//     let element = any_vec.pop().unwrap();
//     println!("{:?}", element.type_id());

//     // # Statically-typed collection of anything
//     use orx_meta::queue::*;

//     let queue = EmptyQueue::new().push(42).push(true).push('x').push("foo");
//     // or
//     let queue = Queue::new(42).push(true).push('x').push("foo");

//     // break down of types
//     let queue: EmptyQueue = EmptyQueue::new();
//     let queue: Queue<i32, EmptyQueue> = queue.push(42);
//     let queue: Queue<i32, Queue<bool, EmptyQueue>> = queue.push(true);
//     let queue: Queue<i32, Queue<bool, Queue<char, EmptyQueue>>> = queue.push('x');
//     let queue: Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>> =
//         queue.push("foo");

//     // using elements
//     let mut queue = Queue::new(42).push(true).push('x').push("foo");

//     let num = queue.front() * 2; // i32
//     assert_eq!(num, 84);

//     *queue.back_mut().front_mut() = false; // bool

//     assert_eq!(queue.back().back().front(), &'x'); // char

//     *queue.back_mut().back_mut().back_mut().front_mut() = "bar"; // &str

//     // shrinking the queue

//     let queue = Queue::new(42).push(true).push('x').push("foo");

//     let (num, queue) = queue.pop();
//     assert_eq!(num, 42);

//     let (flag, queue) = queue.pop();
//     assert_eq!(flag, true);

//     let (ch, queue) = queue.pop();
//     assert_eq!(ch, 'x');

//     let (name, queue) = queue.pop();
//     assert_eq!(name, "foo");

//     // queue.pop(); // doesn't compile; pop does not exist for EmptyQueue!
//     assert!(queue.is_empty());

//     // # Interpretation as an ad-hoc struct

//     struct MyStruct(i32, bool, char, &'static str);
//     let my_struct = MyStruct(42, true, 'x', "foo");

//     type MyQueue = Queue<i32, Queue<bool, Queue<char, Queue<&'static str, EmptyQueue>>>>;
//     let my_queue = Queue::new(42).push(true).push('x').push("foo");

//     type A = Queue<i32, Queue<bool, Queue<char, EmptyQueue>>>;
//     type B = <A as QueueMeta>::PushBack<&'static str>;

//     let a: A = Queue::new(42).push(true).push('x');
//     let b: B = Queue::new(42).push(true).push('x').push("foo");

//     let b: B = a.push("foo"); // B from A
//     // let (name, a): (&'static str, A) = b.pop(); // A from B
// }

fn main() {}
