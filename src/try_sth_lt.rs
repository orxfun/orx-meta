#![allow(dead_code)]

#[macro_export]
macro_rules! define_queue {
    (
        lifetimes => [
            $(
                $g_lt:tt
            )& *
        ];

        generics => [
            $(
                $g:tt:
                $(
                    $g_bnd:ident
                    $(
                        < $( $g_bnd_g:tt ),* >
                    )?
                )| *
            )& *
        ];

        elements => [
            $(
                $el_bnd:ident
                $(
                    < $( $el_bnd_g:tt ),* >
                )?
            )& *
        ];

        names => {
            traits: {
                queue: $q:ident,
                non_empty_queue: $q_ne:ident,
            },
            structs: {
                empty: $empty:ident,
                single: $single:ident,
                pair: $pair:ident,
                composition: $composition:ident,
                builder: $builder:ident,
            },
        };
    ) => {
        // trait: queue

        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
        {
            type PushBack<X>: $q<$($g_lt ,)* $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Back: $q<$($g_lt ,)* $($g ,)*>;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        // trait non-empty queue

        pub trait $q_ne<$($g_lt ,)* $($g ,)*>: $q<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
        {
            fn into_front(self) -> Self::Front;

            fn into_front_back(self) -> (Self::Front, Self::Back);

            fn front(&self) -> &Self::Front;

            fn front_mut(&mut self) -> &mut Self::Front;

            fn back(&self) -> &Self::Back;

            fn back_mut(&mut self) -> &mut Self::Back;

            fn front_back(&self) -> (&Self::Front, &Self::Back);

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back);
        }

        // struct empty

        pub struct $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)*> $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn new() -> Self {
                Self { phantom: Default::default() }
            }
        }

        impl<$($g_lt ,)* $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<X> = $single<$($g_lt ,)* X, $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = $empty<$($g_lt ,)* $($g ,)*>;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $single {
                    phantom: Default::default(),
                    empty: $empty::new(),
                    f: x
                }
            }

            fn len(&self) -> usize {
                0
            }
        }

        // struct single

        pub struct $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            empty: $empty<$($g_lt ,)* $($g ,)*>,
            f: F,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<X> = $pair<$($g_lt ,)* F, $single<$($g_lt ,)* X, $($g ,)*>, $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = $empty<$($g_lt ,)* $($g ,)*>;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair {
                    phantom: Default::default(),
                    f: self.f,
                    b: $single {
                        phantom: Default::default(),
                        empty: $empty::new(),
                        f: x,
                    },
                }
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q_ne<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn into_front(self) -> Self::Front {
                self.f
            }

            fn into_front_back(self) -> (Self::Front, Self::Back) {
                (self.f, $empty{ phantom: Default::default() })
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back(&self) -> &Self::Back {
                &self.empty
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.empty
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.empty)
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.empty)
            }
        }

        // struct pair

        pub struct $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            f: F,
            b: B,
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<X> = $pair<$($g_lt ,)* F, B::PushBack<X>, $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = B;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair {
                    phantom: Default::default(),
                    f: self.f,
                    b: self.b.push_back(x),
                }
            }

            fn len(&self) -> usize {
                1 + self.b.len()
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q_ne<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn into_front(self) -> Self::Front {
                self.f
            }

            fn into_front_back(self) -> (Self::Front, Self::Back) {
                (self.f, self.b)
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back(&self) -> &Self::Back {
                &self.b
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.b
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.b)
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.b)
            }
        }

        // composition

        pub struct $composition<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)*> $composition<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn empty() -> $empty<$($g_lt ,)* $($g ,)*> {
                $empty::new()
            }

            pub fn single<X>(x: X) -> $single<$($g_lt ,)* X, $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
                $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
            {
                $single {
                    phantom: Default::default(),
                    empty: $empty::new(),
                    f: x,
                }
            }

            pub fn compose<C, X>(q: C, x: X) -> C::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
                C: $q<$($g_lt ,)* $($g ,)*>,
            {
                q.push_back(x)
            }
        }

        // builder

        pub struct $builder<$($g_lt ,)* $($g ,)* Rem, Cur>
        where
            Rem:  $q<$($g_lt ,)* $($g ,)*>,
            Cur:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            cur: Cur,
            rem: core::marker::PhantomData<Rem>,
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)* Rem> $builder<$($g_lt ,)* $($g ,)* Rem, $empty<$($g_lt ,)* $($g ,)*>>
        where
            Rem:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
                pub fn new() -> Self {
                    Self {
                        cur: $empty::new(),
                        rem: Default::default(),
                        phantom: Default::default(),
                    }
                }
        }

        impl<$($g_lt ,)* $($g ,)* Rem, Cur> $builder<$($g_lt ,)* $($g ,)* Rem, Cur>
        where
            Rem:  $q<$($g_lt ,)* $($g ,)*>,
            Cur:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn push_back(self, x: Rem::Front) -> $builder<$($g_lt ,)* $($g ,)* Rem::Back, Cur::PushBack<Rem::Front>> {
                $builder {
                    cur: self.cur.push_back(x),
                    rem: Default::default(),
                    phantom: Default::default(),
                }
            }

            pub fn finish(self) -> Cur
            where
                Rem: $q<$($g_lt ,)* $($g ,)* Back = Rem>,
            {
                self.cur
            }
        }



// impl<Rem> Builder<Rem, EmptyQueue>
// where
//     Rem: Queue,
// {
//     pub fn new() -> Self {
//         Self(EmptyQueue, core::marker::PhantomData)
//     }
// }



        // trait $tr<$($g_lt ,)* $($g ,)*>
        // where
        //     $(
        //         $g:
        //         $(
        //             $g_bnd
        //             $(
        //                 <
        //                     $( $g_bnd_g ),*
        //                 >
        //             )?
        //         + ) *
        //     , )*
        //     Self: $(
        //         $el_bnd
        //         $(
        //             < $( $el_bnd_g ),* >
        //         )?
        //     + ) *,
        // {

        //     // abc
        // }
    };
}

pub trait Input {}

pub trait Problem {}
pub struct MyProb;
impl Problem for MyProb {}

pub trait Moves<'a> {
    type P: Problem;
}

pub trait MovesPr<'a, P>: Moves<'a, P = P>
where
    P: Problem,
{
}

mod a {
    use super::*;

    impl<'a, P> Moves<'a> for EmptyQue<'a, P>
    where
        P: Problem,
    {
        type P = P;
    }
    impl<'a, F, P> Moves<'a> for Single<'a, F, P>
    where
        F: Moves<'a>,
        P: Problem,
    {
        type P = P;
    }
    impl<'a, F, B, P> Moves<'a> for Pair<'a, F, B, P>
    where
        F: Moves<'a>,
        B: Que<'a, P>,
        P: Problem,
    {
        type P = P;
    }

    define_queue!(
        lifetimes => ['a];
        generics => [P: Problem];
        elements => [Moves<'a>];
        names => {
            traits: {
                queue: Que,
                non_empty_queue: NonEmptyQue,
            },
            structs: {
                empty: EmptyQue,
                single: Single,
                pair: Pair,
                composition: Comp,
                builder: Bld,
            },
        };
    );
}

mod b {
    define_queue!(
        lifetimes => [];
        generics => [];
        elements => [];
        names => {
            traits: {
                queue: Que,
                non_empty_queue: NonEmptyQue,
            },
            structs: {
                empty: EmptyQue,
                single: Single,
                pair: Pair,
                composition: Comp,
                builder: Bld,
            },
        };
    );
}
