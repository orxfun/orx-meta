use crate::stack::{
    empty::EmptyStack,
    stack::{NonEmptyStack, Stack},
};

#[test]
fn copy() {
    let v0 = 'x';
    let v1 = 32;
    let v2 = String::from("xyz");
    let v3 = true;

    let x = EmptyStack;
    let x = x.push_back(&v0);
    let x = x.push_back(&v1);
    let x = x.push_back(&v2);
    let x = x.push_back(&v3);

    for _ in 0..3 {
        let x = x.clone();

        assert_eq!(x.back(), &&true);
        let (f, x) = x.pop_back();
        assert_eq!(f, &true);

        assert_eq!(x.back(), &&String::from("xyz"));
        let (f, x) = x.pop_back();
        assert_eq!(f, &String::from("xyz"));

        assert_eq!(x.back(), &&32);
        let (f, x) = x.pop_back();
        assert_eq!(f, &32);

        assert_eq!(x.back(), &&'x');
        let (f, x) = x.pop_back();
        assert_eq!(f, &'x');

        assert_eq!(x, EmptyStack);
    }

    assert_eq!(x.len(), 4);
}

#[test]
fn clone() {
    let x = EmptyStack;
    let x = x.push_back('x');
    let x = x.push_back(32);
    let x = x.push_back(String::from("xyz"));
    let x = x.push_back(true);

    for _ in 0..3 {
        let x = x.clone();

        assert_eq!(x.back(), &true);
        let (f, x) = x.pop_back();
        assert_eq!(f, true);

        assert_eq!(x.back(), &String::from("xyz"));
        let (f, x) = x.pop_back();
        assert_eq!(f, String::from("xyz"));

        assert_eq!(x.back(), &32);
        let (f, x) = x.pop_back();
        assert_eq!(f, 32);

        assert_eq!(x.back(), &'x');
        let (f, x) = x.pop_back();
        assert_eq!(f, 'x');

        assert_eq!(x, EmptyStack);
    }

    assert_eq!(x.len(), 4);
}
