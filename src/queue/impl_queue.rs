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

        #[derive(Clone, Copy, Debug)]
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

        #[derive(Clone, Copy, Debug)]
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

        #[derive(Clone, Copy, Debug)]
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

            pub fn single<Elem>(x: Elem) -> $single<$($g_lt ,)* Elem, $($g ,)*>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
                $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
            {
                $single::new(x)
            }

            pub fn compose<Que, Elem>(q: Que, x: Elem) -> Que::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
                Que: $q<$($g_lt ,)* $($g ,)*>,
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

        // tuple support - 1

        impl<$($g_lt ,)* X1, $($g ,)*> $single<$($g_lt ,)* X1, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> X1 {
                self.f
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

        // tuple support - 2

        impl<$($g_lt ,)* X1, X2, $($g ,)*> $pair<$($g_lt ,)* X1, $single<$($g_lt ,)* X2, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, $($g ,)*> From<(X1, X2)> for $pair<$($g_lt ,)* X1, $single<$($g_lt ,)* X2, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2)) -> Self {
                $pair::new(x.0, $single::new(x.1))
            }
        }

        // tuple support - 3

        impl<$($g_lt ,)* X1, X2, X3, $($g ,)*> $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $single<$($g_lt ,)* X3, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.f, self.b.f, self.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, $($g ,)*> From<(X1, X2, X3)> for $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $single<$($g_lt ,)* X3, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $single::new(x.2)))
            }
        }

        // tuple support - 4

        impl<$($g_lt ,)* X1, X2, X3, X4, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $single<$($g_lt ,)* X4, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, $($g ,)*> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $single<$($g_lt ,)* X4, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $single::new(x.3))))
            }
        }

        // tuple support - 5

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $single<$($g_lt ,)* X5, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, $($g ,)*> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $single<$($g_lt ,)* X5, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $single::new(x.4)))))
            }
        }

        // tuple support - 6

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $single<$($g_lt ,)* X6, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, $($g ,)*> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $single<$($g_lt ,)* X6, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $single::new(x.5))))))
            }
        }

        // tuple support - 7

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $single<$($g_lt ,)* X7, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, $($g ,)*> From<(X1, X2, X3, X4, X5, X6, X7)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $single<$($g_lt ,)* X7, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $single::new(x.6)))))))
            }
        }

        // tuple support - 8

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, X8, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $pair<$($g_lt ,)* X7, $single<$($g_lt ,)* X8, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, X8, $($g ,)*> From<(X1, X2, X3, X4, X5, X6, X7, X8)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $pair<$($g_lt ,)* X7, $single<$($g_lt ,)* X8, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $pair::new(x.6, $single::new(x.7))))))))
            }
        }


        // impl<$($g_lt ,)* F, B, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* F, B, $($g ,)*>
        // where
        //     F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
        //     B: $q<$($g_lt ,)* $($g ,)*>,
        //     $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
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

    impl<'a, P> Moves<'a> for EmptyQueue<'a, P>
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
        B: Queue<'a, P>,
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
                queue: Queue,
                non_empty_queue: NonEmptyQueue,
            },
            structs: {
                empty: EmptyQueue,
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
                queue: Queue,
                non_empty_queue: NonEmptyQueue,
            },
            structs: {
                empty: EmptyQueue,
                single: Single,
                pair: Pair,
                composition: Comp,
                builder: Bld,
            },
        };
    );
}
