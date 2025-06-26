use crate::{
    Empty, Multi, One,
    tests::utils::{assert_type, extend, push},
};

#[test]
fn one() {
    let x = Empty;
    let x = extend::<_, One<char>>(x);

    assert_type(&x, "One<char>");
}

#[test]
fn two() {
    let x = Empty;
    let x = extend::<_, One<char>>(x);
    let x = extend::<_, One<u32>>(x);
    assert_type(&x, "Multi<char,One<u32>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, One<u32>>(x);
    assert_type(&x, "Multi<char,One<u32>>");

    let x = Empty;
    let x = extend::<_, Multi<char, One<u32>>>(x);
    assert_type(&x, "Multi<char,One<u32>>");
}

#[test]
fn three() {
    let x = Empty;
    let x = extend::<_, One<char>>(x);
    let x = extend::<_, One<u32>>(x);
    let x = extend::<_, One<String>>(x);
    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");

    let x = Empty;
    let x = extend::<_, Multi<char, Multi<u32, One<String>>>>(x);
    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, Multi<u32, One<String>>>(x);
    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = extend::<_, One<String>>(x);
    assert_type(&x, "Multi<char,Multi<u32,One<String>>>");
}

#[test]
fn four() {
    let x = Empty;
    let x = extend::<_, One<char>>(x);
    let x = extend::<_, One<u32>>(x);
    let x = extend::<_, One<String>>(x);
    let x = extend::<_, One<bool>>(x);
    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");

    let x = Empty;
    let x = extend::<_, Multi<char, Multi<u32, Multi<String, One<bool>>>>>(x);
    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = extend::<_, Multi<u32, Multi<String, One<bool>>>>(x);
    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = extend::<_, Multi<String, One<bool>>>(x);
    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");

    let x = Empty;
    let x = push::<_, char>(x);
    let x = push::<_, u32>(x);
    let x = push::<_, String>(x);
    let x = extend::<_, One<bool>>(x);
    assert_type(&x, "Multi<char,Multi<u32,Multi<String,One<bool>>>>");
}
