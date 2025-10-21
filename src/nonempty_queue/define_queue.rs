#[macro_export]
macro_rules! define_nonempty_queue {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];

        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );

        $crate::define_nonempty_queue_builder!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );

        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
    };

    // # queue_of

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [ ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
    };

    // # builder

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // # queue_of + builder

    // core
    (
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - elements
    (
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };

    // core - lifetime elements
    (
        lt => [$($g_lt:tt), *];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_nonempty_queue_core!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q ; $empty, $pair];
        );
        $crate::define_nonempty_queue_of!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            queue_of => $queue_of;
        );
        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [];
            queue => [$q ; $empty, $pair];
            builder => $builder;
        );
    };
}

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

        impl<$($g_lt ,)* F, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, $empty<$($g_lt ,)* $($g ,)* F>>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn new(element: F) -> $empty<$($g_lt ,)* $($g ,)* F> {
                $empty::new(element)
            }
        }

        impl<$($g_lt ,)* F, B, $($g ,)*> $pair<$($g_lt ,)* $($g ,)* F, B>
        where
            F: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            B: $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
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

// # 3. tuple

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_tuple_transformation {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
    ) => {
        // tuple - 1

        #[allow(dead_code)]
        impl<$($g_lt ,)* X1, $($g ,)*> $empty<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> X1 {
                self.f
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> &X1 {
                &self.f
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> &mut X1 {
                &mut self.f
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1> From<X1> for $empty<$($g_lt ,)* $($g ,)* X1>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: X1) -> Self {
                $empty::new(x)
            }
        }

        // tuple - 2

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2> $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2) {
                (&self.f, &self.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
                (&mut self.f, &mut self.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2> From<(X1, X2)> for $pair<$($g_lt ,)* $($g ,)* X1, $empty<$($g_lt ,)* $($g ,)* X2>>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2)) -> Self {
                $pair::from_fb(x.0, $empty::new(x.1))
            }
        }

        // tuple - 3

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.f, self.b.f, self.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
                (&self.f, &self.b.f, &self.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3> From<(X1, X2, X3)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2, $empty<$($g_lt ,)* $($g ,)* X3>>
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $empty::new(x.2)))
            }
        }

        // tuple - 4

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3, $empty<$($g_lt ,)* $($g ,)* X4>>
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $empty::new(x.3))))
            }
        }

        // tuple - 5

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)* X5>>
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4, $empty<$($g_lt ,)* $($g ,)* X5>>
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $empty::new(x.4)))))
            }
        }

        // tuple - 6

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)* X6>>
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5, $empty<$($g_lt ,)* $($g ,)* X6>>
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $empty::new(x.5))))))
            }
        }

        // tuple - 7

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)* X7>>
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6, &mut X7) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6, $empty<$($g_lt ,)* $($g ,)* X7>>
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $empty::new(x.6)))))))
            }
        }

        // tuple - 8

        #[allow(dead_code)]
        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7, X8>
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6,
                                    $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)* X8>>
                                >
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
                (&self.f, &self.b.f, &self.b.b.f, &self.b.b.b.f, &self.b.b.b.b.f, &self.b.b.b.b.b.f, &self.b.b.b.b.b.b.f, &self.b.b.b.b.b.b.b.f)
            }

            #[inline(always)]
            pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6, &mut X7, &mut X8) {
                (&mut self.f, &mut self.b.f, &mut self.b.b.f, &mut self.b.b.b.f, &mut self.b.b.b.b.f, &mut self.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.f, &mut self.b.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* $($g ,)* X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)> for
            $pair<$($g_lt ,)* $($g ,)* X1,
                $pair<$($g_lt ,)* $($g ,)* X2,
                    $pair<$($g_lt ,)* $($g ,)* X3,
                        $pair<$($g_lt ,)* $($g ,)* X4,
                            $pair<$($g_lt ,)* $($g ,)* X5,
                                $pair<$($g_lt ,)* $($g ,)* X6,
                                    $pair<$($g_lt ,)* $($g ,)* X7, $empty<$($g_lt ,)* $($g ,)* X8>>
                                >
                            >
                        >
                    >
                >
            >
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            #[inline(always)]
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $pair::from_fb(x.0, $pair::from_fb(x.1, $pair::from_fb(x.2, $pair::from_fb(x.3, $pair::from_fb(x.4, $pair::from_fb(x.5, $pair::from_fb(x.6, $empty::new(x.7))))))))
            }
        }
    };
}

// # 4. queue-of

#[doc(hidden)]
#[macro_export]
macro_rules! define_nonempty_queue_of {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident ; $empty:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        macro_rules! $queue_of {
            ($t1:ty) => {
                $empty<$($g_lt ,)* $($g ,)* $t1>
            };

            ($t1:ty, $t2:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1, $empty<$($g_lt ,)* $($g ,)* $t2>>
            };

            ($t1:ty, $t2:ty, $t3:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2, $empty<$($g_lt ,)* $($g ,)* $t3>>
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3, $empty<$($g_lt ,)* $($g ,)* $t4>>
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4, $empty<$($g_lt ,)* $($g ,)* $t5>>
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5, $empty<$($g_lt ,)* $($g ,)* $t6>>
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6, $empty<$($g_lt ,)* $($g ,)* $t7>>
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7, $empty<$($g_lt ,)* $($g ,)* $t8>>
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8, $empty<$($g_lt ,)* $($g ,)* $t9>>
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9, $empty<$($g_lt ,)* $($g ,)* $t10>>
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10, $empty<$($g_lt ,)* $($g ,)* $t11>>
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11, $empty<$($g_lt ,)* $($g ,)* $t12>>
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12, $empty<$($g_lt ,)* $($g ,)* $t13>>
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13, $empty<$($g_lt ,)* $($g ,)* $t14>>
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13,
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14, $empty<$($g_lt ,)* $($g ,)* $t15>>
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty, $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5,
                                    $pair<$($g_lt ,)* $($g ,)* $t6,
                                        $pair<$($g_lt ,)* $($g ,)* $t7,
                                            $pair<$($g_lt ,)* $($g ,)* $t8,
                                                $pair<$($g_lt ,)* $($g ,)* $t9,
                                                    $pair<$($g_lt ,)* $($g ,)* $t10,
                                                        $pair<$($g_lt ,)* $($g ,)* $t11,
                                                            $pair<$($g_lt ,)* $($g ,)* $t12,
                                                                $pair<$($g_lt ,)* $($g ,)* $t13,
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14,
                                                                        $pair<$($g_lt ,)* $($g ,)* $t15, $empty<$($g_lt ,)* $($g ,)* $t16>>
                                                                    >
                                                                >
                                                            >
                                                        >
                                                    >
                                                >
                                            >
                                        >
                                    >
                                >
                            >
                        >
                    >
                >
            };
        }
    };
}
