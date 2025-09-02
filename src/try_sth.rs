#![allow(dead_code)]

pub trait Pr {
    type Z;
    type Y;
}

pub trait Req {
    type Pr: Pr;
}

macro_rules! define_queue3 {
    (
        elements => [$(
                        $x_rhs:tt<[$($x_rhs_type:tt = $x_rhs_type_val:tt), *]>
                    )| *]
        ;

        traits => $q:ident, $ne_q:ident <$($q_tp:ident), *>
        where
            $(
                $q_tp_lhs:ident: [$(
                    $q_tp_rhs:tt<[$($q_tp_rhs_type:tt = $q_tp_rhs_type_val:tt), *]>
                )| *]
            ), *
        ;

        never => $never:tt;

        implementors => $empty:ident, $single:ident, $pair:ident;

    ) => {
        // traits
        pub trait $q<$($q_tp, )*>: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*
        where
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            // types

            type PushBack<X>: $q<$($q_tp, )*>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;

            type Front: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;

            type Back: $q<$($q_tp, )*>;

            // methods

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;
        }

        pub trait $ne_q<$($q_tp, )*>: $q<$($q_tp, )*>
        where
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            fn into_front(self) -> Self::Front;

            fn pop_front(self) -> (Self::Front, Self::Back);

            fn front(&self) -> &Self::Front;

            fn front_back(&self) -> (&Self::Front, &Self::Back);
        }

        // empty

        // #[derive(Clone, Copy, Debug, Default)]
        pub struct $empty<$($q_tp, )*>
        where
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            #[allow(unused_parens)]
            phantom: core::marker::PhantomData<($($q_tp), *)>
        }

        impl<$($q_tp, )*> $q<$($q_tp, )*> for $empty<$($q_tp, )*>
        where
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            type PushBack<X>
                = $single<X, $($q_tp, )*>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;

            type Front = $never<$($q_tp, )*>;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*
            {
                $single {
                    f: x,
                    phantom: Default::default(),
                }
            }
        }

        // single

        // #[derive(Clone, Copy, Debug)]
        pub struct $single<F, $($q_tp, )*>
        where
            F: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*,
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            f: F,
            #[allow(unused_parens)]
            phantom: core::marker::PhantomData<($($q_tp), *)>
        }

        impl<F, $($q_tp, )*> $q<$($q_tp, )*> for $single<F, $($q_tp, )*>
        where
            F: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*,
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            type PushBack<X>
                = $pair<F, $single<X, $($q_tp, )*>, $($q_tp, )*>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;

            type Front = $never<$($q_tp, )*>;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*
            {
                $pair {
                    f: self.f,
                    b: $single {
                        f: x,
                        phantom: Default::default(),
                    },
                    phantom: Default::default(),
                }
            }
        }

        // pair

        // #[derive(Clone, Copy, Debug)]
        pub struct $pair<F, B, $($q_tp, )*>
        where
            F: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*,
            B: $q<$($q_tp, )*>,
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            f: F,
            b: B,
            #[allow(unused_parens)]
            phantom: core::marker::PhantomData<($($q_tp), *)>
        }

        impl<F, B, $($q_tp, )*> $q<$($q_tp, )*> for $pair<F, B, $($q_tp, )*>
        where
            F: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*,
            B: $q<$($q_tp, )*>,
            $($q_tp_lhs: $($q_tp_rhs<$($q_tp_rhs_type = $q_tp_rhs_type_val, )*>+ )*, )*
        {
            type PushBack<X>
                = $pair<F, B::PushBack<X>, $($q_tp, )*>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*;

            type Front = $never<$($q_tp, )*>;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $($x_rhs<$($x_rhs_type = $x_rhs_type_val, )*>+ )*
            {
                $pair {
                    f: self.f,
                    b: self.b.push_back(x),
                    phantom: Default::default(),
                }
            }
        }
    };
}

mod a {
    pub enum Never {}

    define_queue3!(
        elements => [];
        traits => Que00, NonEmptyQue00 <> where;
        never => Never;
        implementors => EmptyQueue00, Single00, Pair00;
    );
}

mod b {
    #[derive(Clone, Copy)]
    pub enum Never {}

    impl Clone for EmptyQueue11 {
        fn clone(&self) -> Self {
            Self {
                phantom: self.phantom.clone(),
            }
        }
    }
    impl Copy for EmptyQueue11 {}
    impl<F: Copy> Clone for Single11<F> {
        fn clone(&self) -> Self {
            Self {
                f: self.f.clone(),
                phantom: self.phantom.clone(),
            }
        }
    }
    impl<F: Copy> Copy for Single11<F> {}
    impl<F: Copy, B: Que11> Clone for Pair11<F, B> {
        fn clone(&self) -> Self {
            Self {
                f: self.f.clone(),
                b: self.b.clone(),
                phantom: self.phantom.clone(),
            }
        }
    }
    impl<F: Copy, B: Que11> Copy for Pair11<F, B> {}

    define_queue3!(
        elements => [Copy<[]>];
        traits => Que11, NonEmptyQue11 <> where;
        never => Never;
        implementors => EmptyQueue11, Single11, Pair11;
    );
}

mod c {
    use super::*;
    mod sealed {
        pub struct NeverWrapper<T>(T);
    }
    pub enum Never<P>
    where
        P: Pr,
    {
        Never(sealed::NeverWrapper<P>),
    }
    impl<P> Req for Never<P>
    where
        P: Pr,
    {
        type Pr = P;
    }
    impl<P: Pr<Y = char>> Req for EmptyQueue<P> {
        type Pr = P;
    }
    impl<F, P: Pr<Y = char>> Req for Single<F, P>
    where
        F: Req<Pr = P>,
    {
        type Pr = P;
    }
    impl<F, B, P: Pr<Y = char>> Req for Pair<F, B, P>
    where
        F: Req<Pr = P>,
        B: Que<P>,
    {
        type Pr = P;
    }

    define_queue3!(
        elements => [Req<[Pr = P]>];
        traits => Que, NonEmptyQue <P> where P: [Pr<[Y = char]>];
        never => Never;
        implementors => EmptyQueue, Single, Pair;
    );
}

// define_queue2!([], [], [], Que1, [], Req);

// define_queue2!(
//     Que1<[]>,
//     PushBack<X>,

//     // oo
//     Req
// );

// define_queue2!(
//     Que2<[P: [Pr<[]>]]>,
//     PushBack<X>,

//     // queue
//     // [P, Z], // cannot be X, F or B
//     // [P: Pr<[Z = Z]>],
//     // Que2,
//     // // push-back
//     // [Req<[Pr = P]>],

//     // oooo
//     Req);

// fn x<P: Pr>(x: impl Que<P>) {}

// pub trait Pr {}
// impl Pr for char {}
// impl Pr for u32 {}
// impl Pr for bool {}

// mod sealed {
//     pub struct NeverWrapper<T>(T);
// }

// pub trait Req {
//     type Pr: Pr;
// }

// pub enum Never<P>
// where
//     P: Pr,
// {
//     Never(sealed::NeverWrapper<P>),
// }

// impl<P> Req for Never<P>
// where
//     P: Pr,
// {
//     type Pr = P;
// }

// // impl

// // traits

// trait Queue<P>
// where
//     P: Pr,
// {
//     type PushBack<X>: NonEmptyQueue<P>
//     where
//         X: Req<Pr = P>;

//     type Front: Req<Pr = P>;

//     type Back: Queue<P>;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Req<Pr = P>;

//     fn len(&self) -> usize;

//     fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }

// trait NonEmptyQueue<P>: Queue<P>
// where
//     P: Pr,
// {
//     fn into_front(self) -> Self::Front;

//     fn pop_front(self) -> (Self::Front, Self::Back);

//     fn front(&self) -> &Self::Front;

//     fn front_back(&self) -> (&Self::Front, &Self::Back);
// }

// // impl: empty

// #[derive(Clone, Copy, Debug, Default)]
// struct EmptyQueue;

// impl<P> Queue<P> for EmptyQueue
// where
//     P: Pr,
// {
//     type PushBack<X>
//         = Single<X>
//     where
//         X: Req<Pr = P>;

//     type Front = Never<P>;

//     type Back = Self;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Req<Pr = P>,
//     {
//         Single(x)
//     }

//     fn len(&self) -> usize {
//         0
//     }
// }

// // impl: single

// #[derive(Clone, Copy, Debug)]
// struct Single<F>(F)
// where
//     F: Req;

// impl<P, F> Queue<P> for Single<F>
// where
//     P: Pr,
//     F: Req<Pr = P>,
// {
//     type PushBack<X>
//         = Pair<F, Single<X>>
//     where
//         X: Req<Pr = P>;

//     type Front = F;

//     type Back = EmptyQueue;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Req<Pr = P>,
//     {
//         Pair(self.0, Single(x))
//     }

//     fn len(&self) -> usize {
//         1
//     }
// }

// impl<P, F> NonEmptyQueue<P> for Single<F>
// where
//     P: Pr,
//     F: Req<Pr = P>,
// {
//     fn into_front(self) -> Self::Front {
//         self.0
//     }

//     fn pop_front(self) -> (Self::Front, Self::Back) {
//         (self.0, EmptyQueue)
//     }

//     fn front(&self) -> &Self::Front {
//         &self.0
//     }

//     fn front_back(&self) -> (&Self::Front, &Self::Back) {
//         (&self.0, &EmptyQueue)
//     }
// }

// // impl: pair

// #[derive(Clone, Copy, Debug)]
// struct Pair<F, B>(F, B)
// where
//     F: Req,
//     B: Queue<F::Pr>;

// impl<F, B> Queue<F::Pr> for Pair<F, B>
// where
//     F: Req,
//     B: Queue<F::Pr>,
// {
//     type PushBack<X>
//         = Pair<F, B::PushBack<X>>
//     where
//         X: Req<Pr = F::Pr>;

//     type Front = F;

//     type Back = B;

//     fn push_back<X>(self, x: X) -> Self::PushBack<X>
//     where
//         X: Req<Pr = F::Pr>,
//     {
//         Pair(self.0, self.1.push_back(x))
//     }

//     fn len(&self) -> usize {
//         1 + self.1.len()
//     }
// }

// impl<F, B> NonEmptyQueue<F::Pr> for Pair<F, B>
// where
//     F: Req,
//     B: Queue<F::Pr>,
// {
//     fn into_front(self) -> Self::Front {
//         self.0
//     }

//     fn pop_front(self) -> (Self::Front, Self::Back) {
//         (self.0, self.1)
//     }

//     fn front(&self) -> &Self::Front {
//         &self.0
//     }

//     fn front_back(&self) -> (&Self::Front, &Self::Back) {
//         (&self.0, &self.1)
//     }
// }

// // test

// struct A(u32);
// struct B(u32);
// struct C(u32);
// impl Req for A {
//     type Pr = char;
// }
// impl Req for B {
//     type Pr = char;
// }
// impl Req for C {
//     type Pr = char;
// }

// #[test]
// fn abc() {
//     let b = EmptyQueue;
//     let b = b.push_back(A(12));
//     let b = b.push_back(B(12));
//     let _b = b.push_back(C(12));
// }
