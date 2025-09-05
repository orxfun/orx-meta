use crate::define_queue_core;

#[test]
fn abc() {
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::new()
        .push_back('x')
        .push_back(12)
        .push_back(true)
        .push_back(0u8)
        .push_back(false);
    // Pr<char, Pr<i32, Pr<bool, Pr<u8, Sng<bool>>>>>

    let p = q.raise();
    // Pr<Sng<char>, Pr<Sng<i32>, Pr<Sng<bool>, Pr<Sng<u8>, Sng<Sng<bool>>>>>>
}

#[test]
fn plain() {
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::new().push_back('x').push_back(12).push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_lt() {
    define_queue_core!(
        lt => ['a];
        generics => [];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::new().push_back('x').push_back(12).push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_gen() {
    define_queue_core!(
        lt => [];
        generics => [T];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::<usize>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_gens() {
    define_queue_core!(
        lt => [];
        generics => [T, U];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::<usize, char>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_gens_with_bounds() {
    define_queue_core!(
        lt => [];
        generics => [T, U: Clone, V: Default | Clone];
        elements => [];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::<usize, char, bool>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
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

    let q = Em::<usize, char, bool>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_elem() {
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [Copy];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::new().push_back('x').push_back(12).push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_elems() {
    use core::fmt::Debug;
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [Copy | Clone | Debug];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );

    let q = Em::new().push_back('x').push_back(12).push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}

#[test]
fn with_elem_bnd() {
    define_queue_core!(
        lt => [];
        generics => [];
        elements => [Into<u64>];
        queue => [ Q, NeQ ; Em, Sng, Pr ];
    );
    impl Into<u64> for Em {
        fn into(self) -> u64 {
            0
        }
    }
    impl<F: Into<u64>> Into<u64> for Sng<F> {
        fn into(self) -> u64 {
            self.f.into()
        }
    }
    impl<F: Into<u64>, B: Q> Into<u64> for Pr<F, B> {
        fn into(self) -> u64 {
            self.f.into() + self.b.into()
        }
    }

    let q = Em::new().push_back(4u32).push_back(12u32).push_back(3u8);
    assert_eq!(q.front(), &4);
    assert_eq!(q.len(), 3);
    let sum: u64 = q.into();
    assert_eq!(sum, 4 + 12 + 3);
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

    let q = Em::<char>::new()
        .push_back('x')
        .push_back(12)
        .push_back(true);
    assert_eq!(q.front(), &'x');
    assert_eq!(q.len(), 3);
}
