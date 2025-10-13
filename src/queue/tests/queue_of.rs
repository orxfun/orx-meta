use crate::{define_queue_core, define_queue_of};

#[test]
fn plain() {
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => [];
        generics => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    let _: qof!() = Em::new();
    let _: qof!(usize) = Em::new().push(2);
    let _: qof!(usize, char) = Em::new().push(2).push('x');
    let _: qof!(usize, char, bool) = Em::new().push(2).push('x').push(true);
    let _: qof!(usize, char, bool, i32) = Em::new().push(2).push('x').push(true).push(3);
    let _: qof!(usize, char, bool, i32, bool) =
        Em::new().push(2).push('x').push(true).push(3).push(true);
    let _: qof!(usize, char, bool, i32, bool, u64) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);
    let _: qof!(usize, char, bool, i32, bool, u64, char) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');
    let _: qof!(usize, char, bool, i32, bool, u64, char, u8) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_lt_omit() {
    define_queue_core!(
        lt => ['a];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => [];
        generics => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    let _: qof!() = Em::new();
    let _: qof!(usize) = Em::new().push(2);
    let _: qof!(usize, char) = Em::new().push(2).push('x');
    let _: qof!(usize, char, bool) = Em::new().push(2).push('x').push(true);
    let _: qof!(usize, char, bool, i32) = Em::new().push(2).push('x').push(true).push(3);
    let _: qof!(usize, char, bool, i32, bool) =
        Em::new().push(2).push('x').push(true).push(3).push(true);
    let _: qof!(usize, char, bool, i32, bool, u64) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);
    let _: qof!(usize, char, bool, i32, bool, u64, char) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');
    let _: qof!(usize, char, bool, i32, bool, u64, char, u8) = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_lt() {
    define_queue_core!(
        lt => ['a];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => ['a];
        generics => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<'a> = qof!();
    let _: Q0<'_> = Em::new();

    type Q1<'a> = qof!(usize);
    let _: Q1<'_> = Em::new().push(2);

    type Q2<'a> = qof!(usize, char);
    let _: Q2<'_> = Em::new().push(2).push('x');

    type Q3<'a> = qof!(usize, char, bool);
    let _: Q3<'_> = Em::new().push(2).push('x').push(true);

    type Q4<'a> = qof!(usize, char, bool, i32);
    let _: Q4<'_> = Em::new().push(2).push('x').push(true).push(3);

    type Q5<'a> = qof!(usize, char, bool, i32, bool);
    let _: Q5<'_> = Em::new().push(2).push('x').push(true).push(3).push(true);

    type Q6<'a> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<'_> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<'a> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<'_> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<'a> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<'_> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_gen() {
    define_queue_core!(
        lt => [];
        generics => [T];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => [];
        generics => [T];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<T> = qof!();
    let _: Q0<_> = Em::<usize>::new();

    type Q1<T> = qof!(usize);
    let _: Q1<_> = Em::<usize>::new().push(2);

    type Q2<T> = qof!(usize, char);
    let _: Q2<_> = Em::<usize>::new().push(2).push('x');

    type Q3<T> = qof!(usize, char, bool);
    let _: Q3<_> = Em::<usize>::new().push(2).push('x').push(true);

    type Q4<T> = qof!(usize, char, bool, i32);
    let _: Q4<_> = Em::<usize>::new().push(2).push('x').push(true).push(3);

    type Q5<T> = qof!(usize, char, bool, i32, bool);
    let _: Q5<_> = Em::<usize>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true);

    type Q6<T> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<_> = Em::<usize>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<T> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<_> = Em::<usize>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<T> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<_> = Em::<usize>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_gens() {
    define_queue_core!(
        lt => [];
        generics => [T, U];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => [];
        generics => [T, U];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<T, U> = qof!();
    let _: Q0<usize, char> = Em::new();

    type Q1<T, U> = qof!(usize);
    let _: Q1<usize, char> = Em::new().push(2);

    type Q2<T, U> = qof!(usize, char);
    let _: Q2<usize, char> = Em::new().push(2).push('x');

    type Q3<T, U> = qof!(usize, char, bool);
    let _: Q3<usize, char> = Em::new().push(2).push('x').push(true);

    type Q4<T, U> = qof!(usize, char, bool, i32);
    let _: Q4<usize, char> = Em::new().push(2).push('x').push(true).push(3);

    type Q5<T, U> = qof!(usize, char, bool, i32, bool);
    let _: Q5<usize, char> = Em::new().push(2).push('x').push(true).push(3).push(true);

    type Q6<T, U> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<usize, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<T, U> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<usize, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<T, U> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<usize, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_gens_with_bounds() {
    define_queue_core!(
        lt => [];
        generics => [T, U: Clone, V: Default | Clone];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    define_queue_of!(
        lt => [];
        generics => [T, U: Clone, V: Default | Clone];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<T, U, V> = qof!();
    let _: Q0<usize, char, bool> = Em::new();

    type Q1<T, U, V> = qof!(usize);
    let _: Q1<usize, char, bool> = Em::new().push(2);

    type Q2<T, U, V> = qof!(usize, char);
    let _: Q2<usize, char, bool> = Em::new().push(2).push('x');

    type Q3<T, U, V> = qof!(usize, char, bool);
    let _: Q3<usize, char, bool> = Em::new().push(2).push('x').push(true);

    type Q4<T, U, V> = qof!(usize, char, bool, i32);
    let _: Q4<usize, char, bool> = Em::new().push(2).push('x').push(true).push(3);

    type Q5<T, U, V> = qof!(usize, char, bool, i32, bool);
    let _: Q5<usize, char, bool> = Em::new().push(2).push('x').push(true).push(3).push(true);

    type Q6<T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_lt_and_gens() {
    pub trait Req<'a> {}
    impl<'a> Req<'a> for usize {}

    define_queue_core!(
        lt => ['a];
        generics => [T: Req<'a>, U, V: Default];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    define_queue_of!(
        lt => ['a];
        generics => [T: Req<'a>, U, V: Default];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<'a, T, U, V> = qof!();
    let _: Q0<'_, usize, char, bool> = Em::new();

    type Q1<'a, T, U, V> = qof!(usize);
    let _: Q1<'_, usize, char, bool> = Em::new().push(2);

    type Q2<'a, T, U, V> = qof!(usize, char);
    let _: Q2<'_, usize, char, bool> = Em::new().push(2).push('x');

    type Q3<'a, T, U, V> = qof!(usize, char, bool);
    let _: Q3<'_, usize, char, bool> = Em::new().push(2).push('x').push(true);

    type Q4<'a, T, U, V> = qof!(usize, char, bool, i32);
    let _: Q4<'_, usize, char, bool> = Em::new().push(2).push('x').push(true).push(3);

    type Q5<'a, T, U, V> = qof!(usize, char, bool, i32, bool);
    let _: Q5<'_, usize, char, bool> = Em::new().push(2).push('x').push(true).push(3).push(true);

    type Q6<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<'_, usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<'_, usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<'_, usize, char, bool> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}

#[test]
fn with_all() {
    pub trait Req<'a, 'b> {}
    pub trait Req2<'a, 'b, T> {}

    define_queue_core!(
        lt => ['a, 'b];
        generics => [T: Copy];
        elements => [Req<'a, 'b> | Req2<'a, 'b, T>];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    impl<'a, 'b, T: Copy> Req<'a, 'b> for Em<'a, 'b, T> {}
    impl<'a, 'b, T: Copy, F: Req<'a, 'b> + Req2<'a, 'b, T>> Req<'a, 'b> for Sng<'a, 'b, T, F> {}
    impl<'a, 'b, T: Copy, F: Req<'a, 'b> + Req2<'a, 'b, T>, B: Q<'a, 'b, T>> Req<'a, 'b>
        for Pr<'a, 'b, T, F, B>
    {
    }

    impl<'a, 'b, T: Copy> Req2<'a, 'b, T> for Em<'a, 'b, T> {}
    impl<'a, 'b, T: Copy, F: Req<'a, 'b> + Req2<'a, 'b, T>> Req2<'a, 'b, T> for Sng<'a, 'b, T, F> {}
    impl<'a, 'b, T: Copy, F: Req<'a, 'b> + Req2<'a, 'b, T>, B: Q<'a, 'b, T>> Req2<'a, 'b, T>
        for Pr<'a, 'b, T, F, B>
    {
    }

    impl<'a, 'b> Req<'a, 'b> for char {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for char {}
    impl<'a, 'b> Req<'a, 'b> for bool {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for bool {}
    impl<'a, 'b> Req<'a, 'b> for u32 {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for u32 {}
    impl<'a, 'b> Req<'a, 'b> for usize {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for usize {}
    impl<'a, 'b> Req<'a, 'b> for i32 {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for i32 {}
    impl<'a, 'b> Req<'a, 'b> for u64 {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for u64 {}
    impl<'a, 'b> Req<'a, 'b> for u8 {}
    impl<'a, 'b, T> Req2<'a, 'b, T> for u8 {}

    define_queue_of!(
        lt => ['a, 'b];
        generics => [T: Copy];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        queue_of => qof;
    );

    type Q0<'a, 'b, T> = qof!();
    let _: Q0<'_, '_, char> = Em::new();

    type Q1<'a, 'b, T> = qof!(usize);
    let _: Q1<'_, '_, char> = Em::new().push(2);

    type Q2<'a, 'b, T> = qof!(usize, char);
    let _: Q2<'_, '_, char> = Em::new().push(2).push('x');

    type Q3<'a, 'b, T> = qof!(usize, char, bool);
    let _: Q3<'_, '_, char> = Em::new().push(2).push('x').push(true);

    type Q4<'a, 'b, T> = qof!(usize, char, bool, i32);
    let _: Q4<'_, '_, char> = Em::new().push(2).push('x').push(true).push(3);

    type Q5<'a, 'b, T> = qof!(usize, char, bool, i32, bool);
    let _: Q5<'_, '_, char> = Em::new().push(2).push('x').push(true).push(3).push(true);

    type Q6<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64);
    let _: Q6<'_, '_, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55);

    type Q7<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char);
    let _: Q7<'_, '_, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y');

    type Q8<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char, u8);
    let _: Q8<'_, '_, char> = Em::new()
        .push(2)
        .push('x')
        .push(true)
        .push(3)
        .push(true)
        .push(55)
        .push('y')
        .push(4);
}
