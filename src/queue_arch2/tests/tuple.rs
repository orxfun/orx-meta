use crate::{define_queue_core, define_queue_of, define_queue_tuple_transformation};

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
    define_queue_tuple_transformation!(
        lt => [];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    type Q1 = qof!(usize);
    type Q2 = qof!(usize, char);
    type Q3 = qof!(usize, char, bool);
    type Q4 = qof!(usize, char, bool, i32);
    type Q5 = qof!(usize, char, bool, i32, bool);
    type Q6 = qof!(usize, char, bool, i32, bool, u64);
    type Q7 = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8 = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let q: Q1 = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2 = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3 = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4 = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5 = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6 = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7 = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8 = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<'a> = qof!(usize);
    type Q2<'a> = qof!(usize, char);
    type Q3<'a> = qof!(usize, char, bool);
    type Q4<'a> = qof!(usize, char, bool, i32);
    type Q5<'a> = qof!(usize, char, bool, i32, bool);
    type Q6<'a> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => ['a];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q: Q1 = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2 = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3 = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4 = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5 = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6 = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7 = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8 = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<T> = qof!(usize);
    type Q2<T> = qof!(usize, char);
    type Q3<T> = qof!(usize, char, bool);
    type Q4<T> = qof!(usize, char, bool, i32);
    type Q5<T> = qof!(usize, char, bool, i32, bool);
    type Q6<T> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => [];
        generics => [T];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q: Q1<usize> = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2<char> = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3<char> = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4<char> = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5<char> = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6<char> = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7<char> = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8<char> = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<T, U> = qof!(usize);
    type Q2<T, U> = qof!(usize, char);
    type Q3<T, U> = qof!(usize, char, bool);
    type Q4<T, U> = qof!(usize, char, bool, i32);
    type Q5<T, U> = qof!(usize, char, bool, i32, bool);
    type Q6<T, U> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T, U> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T, U> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => [];
        generics => [T, U];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q: Q1<char, bool> = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2<char, bool> = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3<char, bool> = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4<char, bool> = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5<char, bool> = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6<char, bool> = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7<char, bool> = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8<char, bool> = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<T, U, V> = qof!(usize);
    type Q2<T, U, V> = qof!(usize, char);
    type Q3<T, U, V> = qof!(usize, char, bool);
    type Q4<T, U, V> = qof!(usize, char, bool, i32);
    type Q5<T, U, V> = qof!(usize, char, bool, i32, bool);
    type Q6<T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => [];
        generics => [T, U: Clone, V: Default | Clone];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q: Q1<char, bool, u8> = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2<char, bool, u8> = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3<char, bool, u8> = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4<char, bool, u8> = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5<char, bool, u8> = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6<char, bool, u8> = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7<char, bool, u8> = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8<char, bool, u8> = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<'a, T, U, V> = qof!(usize);
    type Q2<'a, T, U, V> = qof!(usize, char);
    type Q3<'a, T, U, V> = qof!(usize, char, bool);
    type Q4<'a, T, U, V> = qof!(usize, char, bool, i32);
    type Q5<'a, T, U, V> = qof!(usize, char, bool, i32, bool);
    type Q6<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => ['a];
        generics => [T: Req<'a>, U, V: Default];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    impl<'a> Req<'a> for char {}
    impl<'a> Req<'a> for bool {}
    impl<'a> Req<'a> for u8 {}

    let q: Q1<char, bool, u8> = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2<char, bool, u8> = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3<char, bool, u8> = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4<char, bool, u8> = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5<char, bool, u8> = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6<char, bool, u8> = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7<char, bool, u8> = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8<char, bool, u8> = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
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

    type Q1<'a, 'b, T> = qof!(usize);
    type Q2<'a, 'b, T> = qof!(usize, char);
    type Q3<'a, 'b, T> = qof!(usize, char, bool);
    type Q4<'a, 'b, T> = qof!(usize, char, bool, i32);
    type Q5<'a, 'b, T> = qof!(usize, char, bool, i32, bool);
    type Q6<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    define_queue_tuple_transformation!(
        lt => ['a, 'b];
        generics => [T: Copy];
        elements => [Req<'a, 'b> | Req2<'a, 'b, T>];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q: Q1<char> = 2usize.into();
    assert_eq!(q.as_tuple(), &2);

    let mut q: Q2<char> = (2, 'x').into();
    *q.as_tuple_mut().1 = 'y';
    assert_eq!(q.as_tuple(), (&2, &'y'));

    let q: Q3<char> = (2, 'x', true).into();
    assert_eq!(q.as_tuple(), (&2, &'x', &true));

    let q: Q4<char> = (2, 'x', true, 4).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4));

    let q: Q5<char> = (2, 'x', true, 4, false).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false));

    let q: Q6<char> = (2, 'x', true, 4, false, 7).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7));

    let q: Q7<char> = (2, 'x', true, 4, false, 7, 'y').into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y'));

    let q: Q8<char> = (2, 'x', true, 4, false, 7, 'y', 0).into();
    assert_eq!(q.into_tuple(), (2, 'x', true, 4, false, 7, 'y', 0));
}
