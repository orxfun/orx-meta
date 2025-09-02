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

mod d {
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

// mod a {
//     pub enum Never {}

//     define_queue3!(
//         elements => [];
//         traits => Que00, NonEmptyQue00 <> where;
//         never => Never;
//         implementors => EmptyQueue00, Single00, Pair00;
//     );
// }

// mod b {
//     #[derive(Clone, Copy)]
//     pub enum Never {}

//     impl Clone for EmptyQueue11 {
//         fn clone(&self) -> Self {
//             Self {
//                 phantom: self.phantom.clone(),
//             }
//         }
//     }
//     impl Copy for EmptyQueue11 {}
//     impl<F: Copy> Clone for Single11<F> {
//         fn clone(&self) -> Self {
//             Self {
//                 f: self.f.clone(),
//                 phantom: self.phantom.clone(),
//             }
//         }
//     }
//     impl<F: Copy> Copy for Single11<F> {}
//     impl<F: Copy, B: Que11> Clone for Pair11<F, B> {
//         fn clone(&self) -> Self {
//             Self {
//                 f: self.f.clone(),
//                 b: self.b.clone(),
//                 phantom: self.phantom.clone(),
//             }
//         }
//     }
//     impl<F: Copy, B: Que11> Copy for Pair11<F, B> {}

//     define_queue3!(
//         lifetimes => [];
//         elements => [Copy<[]>];
//         traits => Que11, NonEmptyQue11 <> where;
//         never => Never;
//         implementors => EmptyQueue11, Single11, Pair11;
//     );
// }

// mod c {
//     use super::*;
//     mod sealed {
//         pub struct NeverWrapper<T>(T);
//     }
//     pub enum Never<P>
//     where
//         P: Pr,
//     {
//         Never(sealed::NeverWrapper<P>),
//     }
//     impl<P> Req for Never<P>
//     where
//         P: Pr,
//     {
//         type Pr = P;
//     }
//     impl<P: Pr<Y = char>> Req for EmptyQueue<P> {
//         type Pr = P;
//     }
//     impl<F, P: Pr<Y = char>> Req for Single<F, P>
//     where
//         F: Req<Pr = P>,
//     {
//         type Pr = P;
//     }
//     impl<F, B, P: Pr<Y = char>> Req for Pair<F, B, P>
//     where
//         F: Req<Pr = P>,
//         B: Que<P>,
//     {
//         type Pr = P;
//     }

//     define_queue3!(
//         lifetimes => [];
//         elements => [Req<[Pr = P]>];
//         traits => Que, NonEmptyQue <P> where P: [Pr<[Y = char]>];
//         never => Never;
//         implementors => EmptyQueue, Single, Pair;
//     );
// }
