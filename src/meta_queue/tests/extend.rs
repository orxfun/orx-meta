use super::utils::{assert_type, extend, push};
use crate::meta_queue::{empty::Empty, pair::Pair, single::Single};

#[test]
fn one() {
    let x = Empty;
    let x = extend::<_, Single<char>>(x);

    assert_type(&x, "Single<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = extend::<_, Single<char>>(x);
    let x = extend::<_, Single<u32>>(x);
    assert_type(&x, "Pair<char,Single<u32>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, Single<u32>>(x);
    assert_type(&x, "Pair<char,Single<u32>>");

    let x = Empty;
    let x = extend::<_, Pair<char, Single<u32>>>(x);
    assert_type(&x, "Pair<char,Single<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = extend::<_, Single<char>>(x);
    let x = extend::<_, Single<u32>>(x);
    let x = extend::<_, Single<String>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");

    let x = Empty;
    let x = extend::<_, Pair<char, Pair<u32, Single<String>>>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, Pair<u32, Single<String>>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = extend::<_, Single<String>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Single<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = extend::<_, Single<char>>(x);
    let x = extend::<_, Single<u32>>(x);
    let x = extend::<_, Single<String>>(x);
    let x = extend::<_, Single<bool>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");

    let x = Empty;
    let x = extend::<_, Pair<char, Pair<u32, Pair<String, Single<bool>>>>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, Pair<u32, Pair<String, Single<bool>>>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = extend::<_, Pair<String, Single<bool>>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = extend::<_, Single<bool>>(x);
    assert_type(&x, "Pair<char,Pair<u32,Pair<String,Single<bool>>>>");
}
