#![allow(dead_code)]

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
macro_rules! define_queue {
    (
        names => {traits: {queue: $q:ident,non_empty_queue: $q_ne:ident},structs: {empty: $empty:ident,single: $single:ident,pair: $pair:ident}};
        $(queue_of => $q_of:ident;)?
    ) => {
        define_queue!(
            lifetimes => [];
            generics => [];
            elements => [];
            names => {traits: {queue: $q,non_empty_queue: $q_ne},structs: {empty: $empty,single: $single,pair: $pair}};
            $(queue_of => $q_of;)?
        );
    };

    (
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        names => {traits: {queue: $q:ident,non_empty_queue: $q_ne:ident},structs: {empty: $empty:ident,single: $single:ident,pair: $pair:ident}};
        $(queue_of => $q_of:ident;)?
    ) => {
        define_queue!(
            lifetimes => [];
            generics => [];
            elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];
            names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};
            $(queue_of => $q_of;)?
        );
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        names => {traits: {queue: $q:ident,non_empty_queue: $q_ne:ident},structs: {empty: $empty:ident,single: $single:ident,pair: $pair:ident}};
        $(queue_of => $q_of:ident;)?
    ) => {
        define_queue!(
            lifetimes => [$($g_lt)& *];
            generics => [];
            elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];
            names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};
            $(queue_of => $q_of;)?
        );
    };

    (
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        names => {traits: {queue: $q:ident,non_empty_queue: $q_ne:ident},structs: {empty: $empty:ident,single: $single:ident,pair: $pair:ident}};
        $(queue_of => $q_of:ident;)?
    ) => {
        define_queue!(
            lifetimes => [];
            generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];
            elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];
            names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};
            $(queue_of => $q_of;)?
        );
    };

    // complete implementation
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
                non_empty_queue: $q_ne:ident
            },
            structs: {
                empty: $empty:ident,
                single: $single:ident,
                pair: $pair:ident
            }
        };

        $(
            queue_of => $q_of:ident;
        )?
    ) => {
        // trait: queue

        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
        {
            type PushBack<Elem>: $q_ne<$($g_lt ,)* $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Back: $q<$($g_lt ,)* $($g ,)*>;

            fn push_back<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

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

                fn into_back(self) -> Self::Back;

                fn pop(self) -> (Self::Front, Self::Back);

                fn front(&self) -> &Self::Front;

                fn back(&self) -> &Self::Back;

                fn front_back(&self) -> (&Self::Front, &Self::Back);

                fn front_mut(&mut self) -> &mut Self::Front;

                fn back_mut(&mut self) -> &mut Self::Back;

                fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back);
        }

        // struct empty

        #[derive(Clone, Copy, PartialEq, Eq)]
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

        impl<$($g_lt ,)* $($g ,)*> core::fmt::Debug for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", stringify!($empty))
            }
        }

        impl<$($g_lt ,)* $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)*>
        where
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<Elem> = $single<$($g_lt ,)* Elem, $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = $empty<$($g_lt ,)* $($g ,)*>;

            type Back = Self;

            fn push_back<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $single::new(x)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // struct single

        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            empty: $empty<$($g_lt ,)* $($g ,)*>,
            f: F,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn new(f: F) -> Self {
                Self {
                    phantom: Default::default(),
                    empty: $empty::new(),
                    f,
                }
            }
        }

        impl<$($g_lt ,)* X1, $($g ,)*> From<X1> for $single<$($g_lt ,)* X1, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: X1) -> Self {
                $single::new(x)
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> core::fmt::Debug for $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?})", stringify!($single), self.f)
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $single<$($g_lt ,)* F, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* F, $single<$($g_lt ,)* Elem, $($g ,)*>, $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = $empty<$($g_lt ,)* $($g ,)*>;

            fn push_back<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::new(self.f, $single::new(x))
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

            fn into_back(self) -> Self::Back {
                self.empty
            }

            fn pop(self) -> (Self::Front, Self::Back) {
                (self.f, $empty{ phantom: Default::default() })
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn back(&self) -> &Self::Back {
                &self.empty
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.empty)
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.empty
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.empty)
            }
        }

        // struct pair

        #[derive(Clone, Copy, PartialEq, Eq)]
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

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn new(f: F, b: B) -> Self {
                Self {
                    phantom: Default::default(),
                    f,
                    b,
                }
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> core::fmt::Debug for $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: core::fmt::Debug,
            B: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?}, {:?})", stringify!($pair), self.f, self.b)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* F, B, $($g ,)*>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* F, B::PushBack<Elem>, $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = B;

            fn push_back<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::new(self.f, self.b.push_back(x))
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

            fn into_back(self) -> Self::Back {
                self.b
            }

            fn pop(self) -> (Self::Front, Self::Back) {
                (self.f, self.b)
            }

            fn front(&self) -> &Self::Front {
                &self.f
            }

            fn back(&self) -> &Self::Back {
                &self.b
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.f, &self.b)
            }

            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            fn back_mut(&mut self) -> &mut Self::Back {
                &mut self.b
            }

            fn front_back_mut(&mut self) -> (&mut Self::Front, &mut Self::Back) {
                (&mut self.f, &mut self.b)
            }
        }

        // queue of

        $(
            crate::with_dollar_sign! {
                ($d:tt) => {
                    macro_rules! $q_of {
                        ([$d($qg:tt),*]) => { $empty<$d($qg),*> };
                        ([$d($qg:tt),*], $t1:ty) => { $single<$d($qg),*, $t1> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty) => { $pair<$d($qg),*, $t1, $single<$d($qg),*, $t2>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty) => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $single<$d($qg),*, $t3>>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty, $t4:ty) => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $single<$d($qg),*, $t4>>>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty)
                            => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $single<$d($qg),*, $t5>>>>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty)
                            => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $single<$d($qg),*, $t6>>>>>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty)
                            => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $pair<$d($qg),*, $t6, $single<$d($qg),*, $t7>>>>>>> };
                        ([$d($qg:tt),*], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty)
                            => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $pair<$d($qg),*, $t6, $pair<$d($qg),*, $t7, $single<$d($qg),*, $t8>>>>>>>> };

                        () => { $empty };
                        ($t1:ty) => { $single<$t1> };
                        ($t1:ty, $t2:ty) => { $pair<$t1, $single<$t2>> };
                        ($t1:ty, $t2:ty, $t3:ty) => { $pair<$t1, $pair<$t2, $single<$t3>>> };
                        ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => { $pair<$t1, $pair<$t2, $pair<$t3, $single<$t4>>>> };
                        ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty)
                            => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $single<$t5>>>>> };
                        ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty)
                            => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $single<$t6>>>>>> };
                        ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty)
                            => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $pair<$t6, $single<$t7>>>>>>> };
                        ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty)
                            => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $pair<$t6, $pair<$t7, $single<$t8>>>>>>>> };
                    }
                };
            }
            // macro_rules! $q_of {
            //     () => {
            //         with_dollar_sign! {
            //             ([$($qg:tt &)*]) => {  };
            //         }
            //     };
            //     // () => { $empty };
            //     // ($t1:ty) => { $single<$t1> };
            //     // ($t1:ty, $t2:ty) => { $pair<$t1, $single<$t2>> };
            //     // ($t1:ty, $t2:ty, $t3:ty) => { $pair<$t1, $pair<$t2, $single<$t3>>> };

            //     // ([$($qg:tt &)*]) => {  };
            //     // (@internal ($($qg:tt &)*)) => {};
            // }
        )?
    };
}
//         generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
