use crate::{define_queue_builder, define_queue_core, define_queue_of};

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
    define_queue_builder!(
        lt => [];
        generics => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    trait NewBld: Q + Sized {
        fn builder() -> Bld<Self, Em> {
            Default::default()
        }
    }
    impl<P: Q> NewBld for P {}

    type Q0 = qof!();
    type Q1 = qof!(usize);
    type Q2 = qof!(usize, char);
    type Q3 = qof!(usize, char, bool);
    type Q4 = qof!(usize, char, bool, i32);
    type Q5 = qof!(usize, char, bool, i32, bool);
    type Q6 = qof!(usize, char, bool, i32, bool, u64);
    type Q7 = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8 = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0 = Bld::<Q0, _>::new().finish();
    let _: Q1 = Bld::<Q1, _>::new().push(2).finish();
    let _: Q2 = Bld::<Q2, _>::new().push(2).push('x').finish();
    let _: Q3 = Bld::<Q3, _>::new().push(2).push('x').push(true).finish();
    let _: Q4 = Bld::<Q4, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5 = Bld::<Q5, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6 = Bld::<Q6, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7 = Bld::<Q7, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8 = Bld::<Q8, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();

    let _: Q8 = Q8::builder()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
        lt => ['a];
        generics => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<'a> = qof!();
    type Q1<'a> = qof!(usize);
    type Q2<'a> = qof!(usize, char);
    type Q3<'a> = qof!(usize, char, bool);
    type Q4<'a> = qof!(usize, char, bool, i32);
    type Q5<'a> = qof!(usize, char, bool, i32, bool);
    type Q6<'a> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0 = Bld::<Q0, _>::new().finish();
    let _: Q1 = Bld::<Q1, _>::new().push(2).finish();
    let _: Q2 = Bld::<Q2, _>::new().push(2).push('x').finish();
    let _: Q3 = Bld::<Q3, _>::new().push(2).push('x').push(true).finish();
    let _: Q4 = Bld::<Q4, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5 = Bld::<Q5, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6 = Bld::<Q6, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7 = Bld::<Q7, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8 = Bld::<Q8, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
        lt => [];
        generics => [T];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<T> = qof!();
    type Q1<T> = qof!(usize);
    type Q2<T> = qof!(usize, char);
    type Q3<T> = qof!(usize, char, bool);
    type Q4<T> = qof!(usize, char, bool, i32);
    type Q5<T> = qof!(usize, char, bool, i32, bool);
    type Q6<T> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0<usize> = Bld::<usize, Q0<usize>, _>::new().finish();
    let _: Q1<usize> = Bld::<usize, Q1<usize>, _>::new().push(2).finish();
    let _: Q2<usize> = Bld::<usize, Q2<usize>, _>::new().push(2).push('x').finish();
    let _: Q3<usize> = Bld::<usize, Q3<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .finish();
    let _: Q4<usize> = Bld::<usize, Q4<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5<usize> = Bld::<usize, Q5<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6<usize> = Bld::<usize, Q6<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7<usize> = Bld::<usize, Q7<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8<usize> = Bld::<usize, Q8<usize>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
        lt => [];
        generics => [T, U];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<T, U> = qof!();
    type Q1<T, U> = qof!(usize);
    type Q2<T, U> = qof!(usize, char);
    type Q3<T, U> = qof!(usize, char, bool);
    type Q4<T, U> = qof!(usize, char, bool, i32);
    type Q5<T, U> = qof!(usize, char, bool, i32, bool);
    type Q6<T, U> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T, U> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T, U> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0<usize, char> = Bld::<usize, char, Q0<usize, char>, _>::new().finish();
    let _: Q1<usize, char> = Bld::<usize, char, Q1<usize, char>, _>::new()
        .push(2)
        .finish();
    let _: Q2<usize, char> = Bld::<usize, char, Q2<usize, char>, _>::new()
        .push(2)
        .push('x')
        .finish();
    let _: Q3<usize, char> = Bld::<usize, char, Q3<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .finish();
    let _: Q4<usize, char> = Bld::<usize, char, Q4<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5<usize, char> = Bld::<usize, char, Q5<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6<usize, char> = Bld::<usize, char, Q6<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7<usize, char> = Bld::<usize, char, Q7<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8<usize, char> = Bld::<usize, char, Q8<usize, char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
        lt => [];
        generics => [T, U: Clone, V: Default | Clone];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<T, U, V> = qof!();
    type Q1<T, U, V> = qof!(usize);
    type Q2<T, U, V> = qof!(usize, char);
    type Q3<T, U, V> = qof!(usize, char, bool);
    type Q4<T, U, V> = qof!(usize, char, bool, i32);
    type Q5<T, U, V> = qof!(usize, char, bool, i32, bool);
    type Q6<T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0<usize, char, u8> = Bld::<usize, char, u8, Q0<usize, char, u8>, _>::new().finish();
    let _: Q1<usize, char, u8> = Bld::<usize, char, u8, Q1<usize, char, u8>, _>::new()
        .push(2)
        .finish();
    let _: Q2<usize, char, u8> = Bld::<usize, char, u8, Q2<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .finish();
    let _: Q3<usize, char, u8> = Bld::<usize, char, u8, Q3<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .finish();
    let _: Q4<usize, char, u8> = Bld::<usize, char, u8, Q4<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5<usize, char, u8> = Bld::<usize, char, u8, Q5<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6<usize, char, u8> = Bld::<usize, char, u8, Q6<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7<usize, char, u8> = Bld::<usize, char, u8, Q7<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8<usize, char, u8> = Bld::<usize, char, u8, Q8<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
        lt => ['a];
        generics => [T: Req<'a>, U, V: Default];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<'a, T, U, V> = qof!();
    type Q1<'a, T, U, V> = qof!(usize);
    type Q2<'a, T, U, V> = qof!(usize, char);
    type Q3<'a, T, U, V> = qof!(usize, char, bool);
    type Q4<'a, T, U, V> = qof!(usize, char, bool, i32);
    type Q5<'a, T, U, V> = qof!(usize, char, bool, i32, bool);
    type Q6<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a, T, U, V> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0<usize, char, u8> = Bld::<usize, char, u8, Q0<usize, char, u8>, _>::new().finish();
    let _: Q1<usize, char, u8> = Bld::<usize, char, u8, Q1<usize, char, u8>, _>::new()
        .push(2)
        .finish();
    let _: Q2<usize, char, u8> = Bld::<usize, char, u8, Q2<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .finish();
    let _: Q3<usize, char, u8> = Bld::<usize, char, u8, Q3<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .finish();
    let _: Q4<usize, char, u8> = Bld::<usize, char, u8, Q4<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5<usize, char, u8> = Bld::<usize, char, u8, Q5<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6<usize, char, u8> = Bld::<usize, char, u8, Q6<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7<usize, char, u8> = Bld::<usize, char, u8, Q7<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8<usize, char, u8> = Bld::<usize, char, u8, Q8<usize, char, u8>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
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
    define_queue_builder!(
         lt => ['a, 'b];
        generics => [T: Copy];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
        builder => Bld;
    );

    type Q0<'a, 'b, T> = qof!();
    type Q1<'a, 'b, T> = qof!(usize);
    type Q2<'a, 'b, T> = qof!(usize, char);
    type Q3<'a, 'b, T> = qof!(usize, char, bool);
    type Q4<'a, 'b, T> = qof!(usize, char, bool, i32);
    type Q5<'a, 'b, T> = qof!(usize, char, bool, i32, bool);
    type Q6<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64);
    type Q7<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char);
    type Q8<'a, 'b, T> = qof!(usize, char, bool, i32, bool, u64, char, u8);

    let _: Q0<char> = Bld::<char, Q0<char>, _>::new().finish();
    let _: Q1<char> = Bld::<char, Q1<char>, _>::new().push(2).finish();
    let _: Q2<char> = Bld::<char, Q2<char>, _>::new().push(2).push('x').finish();
    let _: Q3<char> = Bld::<char, Q3<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .finish();
    let _: Q4<char> = Bld::<char, Q4<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .finish();
    let _: Q5<char> = Bld::<char, Q5<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .finish();
    let _: Q6<char> = Bld::<char, Q6<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .finish();
    let _: Q7<char> = Bld::<char, Q7<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .finish();
    let _: Q8<char> = Bld::<char, Q8<char>, _>::new()
        .push(2)
        .push('x')
        .push(true)
        .push(4)
        .push(false)
        .push(4)
        .push('y')
        .push(0)
        .finish();
}
