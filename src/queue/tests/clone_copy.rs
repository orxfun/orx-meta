use crate::queue::{
    empty::EmptyQueue,
    queue::{NonEmptyQueue, Queue},
};

#[test]
fn copy() {
    let v0 = 'x';
    let v1 = 32;
    let v2 = String::from("xyz");
    let v3 = true;

    let x = EmptyQueue;
    let x = x.push_back(&v0);
    let x = x.push_back(&v1);
    let x = x.push_back(&v2);
    let x = x.push_back(&v3);

    for _ in 0..3 {
        let x = x; // copy original

        assert_eq!(x.front(), &&'x');
        let (f, x) = x.pop_front();
        assert_eq!(f, &'x');

        assert_eq!(x.front(), &&32);
        let (f, x) = x.pop_front();
        assert_eq!(f, &32);

        assert_eq!(x.front(), &&String::from("xyz"));
        let (f, x) = x.pop_front();
        assert_eq!(f, &String::from("xyz"));

        assert_eq!(x.front(), &&true);
        let (f, x) = x.pop_front();
        assert_eq!(f, &true);

        assert_eq!(x, EmptyQueue);
    }

    assert_eq!(x.len(), 4);
}

#[test]
fn clone() {
    let x = EmptyQueue;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    for _ in 0..3 {
        let x = x.clone(); // clone original

        assert_eq!(x.front(), &'x');
        let (f, x) = x.pop_front();
        assert_eq!(f, 'x');

        assert_eq!(x.front(), &32);
        let (f, x) = x.pop_front();
        assert_eq!(f, 32);

        assert_eq!(x.front(), &String::from("xyz"));
        let (f, x) = x.pop_front();
        assert_eq!(f, String::from("xyz"));

        assert_eq!(x.front(), &true);
        let (f, x) = x.pop_front();
        assert_eq!(f, true);

        assert_eq!(x, EmptyQueue);
    }

    assert_eq!(x.len(), 4);
}
