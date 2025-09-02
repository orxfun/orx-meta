#![allow(dead_code)]

macro_rules! define {
    (
        $tr:ident;

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
                never: $never:ident,
                empty: $empty:ident,
                single: $single:ident,
                pair: $pair:ident,
            },
        };
    ) => {
        // never
        #[allow(unused_parens)]
        struct NeverWrapper___<$($g_lt ,)* $($g ,)*>(core::marker::PhantomData<&$($g_lt)* ($($g ,)*)>);
        pub enum $never<$($g_lt ,)* $($g ,)*> {
            #[allow(private_interfaces)]
            Never(NeverWrapper___<$($g_lt ,)* $($g ,)*>),
        }

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

            fn front_back(&self) -> (&Self::Front, &Self::Back);

            fn front_mut(&mut self) -> &mut Self::Front;

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back);
        }

        // struct empty

        pub struct $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<&$($g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<X> = $single<$($g_lt ,)* X, $($g ,)*>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = $never<$($g_lt ,)* $($g ,)*>;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $single {
                    phantom: Default::default(),
                    f: x
                }
            }
        }

        // struct single

        pub struct $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<&$($g_lt)* ($($g ,)*)>,
            f: F,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<X> = Self
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = $empty<$($g_lt ,)* $($g ,)*>;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                todo!()
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

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &$empty{ phantom: Default::default() })
                // todo!()
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                todo!()
            }
        }



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

    impl<'a, P> Moves<'a> for Never<'a, P>
    where
        P: Problem,
    {
        type P = P;
    }
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

    define!(
        Ahoy;

        lifetimes => [
            'a
        ];

        generics => [
            P: Problem
        ];

        elements => [
            Moves<'a>
        ];

        names => {
            traits: {
                queue: Que,
                non_empty_queue: NonEmptyQue,
            },
            structs: {
                never: Never,
                empty: EmptyQue,
                single: Single,
                pair: Pair,
            },
        };
    );
}
