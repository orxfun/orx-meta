// # 1. core

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_core {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        #[allow(dead_code)]
        pub trait $q<$($g_lt ,)* $($g ,)*>
        where
            Self: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<T>: $q<$($g_lt ,)* $($g ,)*>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Back: $q<$($g_lt ,)* $($g ,)*>;

            const LEN: usize;

            fn push<T>(self, x: T) -> Self::PushBack<T>
            where
                T: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            fn len(&self) -> usize {
                Self::LEN
            }

            fn front(&self) -> &Self::Front;

            fn front_mut(&mut self) -> &mut Self::Front;

            fn into_front(self) -> Self::Front;
        }

        // # empty

        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $empty<$($g_lt ,)* $($g ,)* Front>
        where
            Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            f: Front,
        }

        impl<$($g_lt ,)* F, $($g ,)*> $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn new(element: F) -> Self {
                Self {
                    phantom: Default::default(),
                    f: element,
                }
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> core::fmt::Debug for $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?})", stringify!($empty), self.f)
            }
        }

        impl<$($g_lt ,)* F, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $empty<$($g_lt ,)* $($g ,)* F>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* F, $empty<$($g_lt ,)* $($g ,)* Elem>>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = Self;

            const LEN: usize = 1;

            #[inline(always)]
            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::from_fb(self.f, $empty::new(x))
            }

            #[inline(always)]
            fn front(&self) -> &Self::Front {
                &self.f
            }

            #[inline(always)]
            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            #[inline(always)]
            fn into_front(self) -> Self::Front {
                self.f
            }
        }

        // # pair

        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $pair<$($g_lt ,)* $($g ,)* Front, Back>
        where
            Front: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            Back: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
            f: Front,
            b: Back,
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn from_fb(f: F, b: B) -> Self {
                Self {
                    phantom: Default::default(),
                    f,
                    b,
                }
            }

            // ref

            pub fn back(&self) -> &B {
                &self.b
            }

            // mut

            pub fn back_mut(&mut self) -> &mut B {
                &mut self.b
            }

            pub fn front_back_mut(&mut self) -> (&mut F, &mut B) {
                (&mut self.f, &mut self.b)
            }

            // into

            pub fn into_back(self) -> B {
                self.b
            }

            pub fn pop(self) -> (F, B) {
                (self.f, self.b)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> core::fmt::Debug for $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: core::fmt::Debug,
            B: core::fmt::Debug,
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?}, {:?})", stringify!($pair), self.f, self.b)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $q<$($g_lt ,)* $($g ,)*> for $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            type PushBack<Elem> = $pair<$($g_lt ,)* $($g ,)* F, B::PushBack<Elem>>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *;

            type Front = F;

            type Back = B;

            const LEN: usize = 1 + B::LEN;

            fn push<Elem>(self, x: Elem) -> Self::PushBack<Elem>
            where
                Elem: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *
            {
                $pair::from_fb(self.f, self.b.push(x))
            }

            #[inline(always)]
            fn front(&self) -> &Self::Front {
                &self.f
            }

            #[inline(always)]
            fn front_mut(&mut self) -> &mut Self::Front {
                &mut self.f
            }

            #[inline(always)]
            fn into_front(self) -> Self::Front {
                self.f
            }
        }
    };
}

// # 2. builder

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_builder {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {

        // builder

        pub struct $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            target: core::marker::PhantomData<Target>,
        }

        impl<$($g_lt ,)* $($g ,)* Target> Default for $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<$($g_lt ,)* $($g ,)* Target> $builder<$($g_lt ,)* $($g ,)* Target>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn new() -> Self {
                Self {
                    target: Default::default(),
                }
            }

            pub fn push(
                self,
                element: Target::Front,
            ) -> QueueBuilding<Target, Target::Back, $empty<$($g_lt ,)* $($g ,)* Target::Front>> {
                QueueBuilding::new($empty::new(element))
            }
        }

        // building

        pub struct QueueBuilding<$($g_lt ,)* $($g ,)* Target, Remaining, Current>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            target: core::marker::PhantomData<Target>,
            remaining: core::marker::PhantomData<Remaining>,
            current: Current,
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)* Target, Remaining, Current> QueueBuilding<$($g_lt ,)* $($g ,)* Target, Remaining, Current>
        where
            Target:  $q<$($g_lt ,)* $($g ,)*>,
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn new(current: Current) -> Self {
                Self {
                    target: Default::default(),
                    remaining: Default::default(),
                    current: current,
                    phantom: Default::default(),
                }
            }

            #[inline(always)]
            pub fn push(
                self,
                element: Remaining::Front,
            ) -> QueueBuilding<Target, Remaining::Back, Current::PushBack<Remaining::Front>> {
                QueueBuilding::new(self.current.push(element))
            }

            #[inline(always)]
            pub fn finish(self) -> Current
            where
                Target: $q<$($g_lt ,)* $($g ,)* Front = Current::Front, Back = Current::Back>,
            {
                self.current
            }
        }
    };
}

#[test]
fn abc() {
    define_nonempty_queue_core!(
        lt => [];
        generics => [];
        elements => [];
        queue => [ StQueue ; Single, Queue ];
    );

    define_nonempty_queue_builder!(
        lt => [];
        generics => [];
        queue => [ StQueue ; Single, Queue ];
        builder => QueueBuilder;
    );

    type Q = Queue<u32, Queue<char, Single<bool>>>;
    let q: Q = QueueBuilder::<Q>::new()
        .push(1)
        .push('x')
        .push(true)
        .finish();
    let a = Single::new(1).push('x').push(true);
}
