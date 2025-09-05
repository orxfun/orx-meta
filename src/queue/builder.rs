#[macro_export]
macro_rules! define_queue_builder {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];
        builder => $builder:ident;
    ) => {
        pub struct $builder<$($g_lt ,)* $($g ,)* Remaining, Current>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            cur: Current,
            rem: core::marker::PhantomData<Remaining>,
            phantom: core::marker::PhantomData<$(&$g_lt)* ($($g ,)*)>,
        }

        impl<$($g_lt ,)* $($g ,)* Remaining> $builder<$($g_lt ,)* $($g ,)* Remaining, $empty<$($g_lt ,)* $($g ,)*>>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
                pub fn new() -> Self {
                    Self {
                        cur: $empty::new(),
                        rem: Default::default(),
                        phantom: Default::default(),
                    }
                }
        }

        impl<$($g_lt ,)* $($g ,)* Remaining, Current> $builder<$($g_lt ,)* $($g ,)* Remaining, Current>
        where
            Remaining:  $q<$($g_lt ,)* $($g ,)*>,
            Current:  $q<$($g_lt ,)* $($g ,)*>,
            $( $g: $( $( $g_bnd $( < $( $g_bnd_g ),* > )? + )* )? ), *
        {
            pub fn push_back(self, x: Remaining::Front) -> $builder<$($g_lt ,)* $($g ,)* Remaining::Back, Current::PushBack<Remaining::Front>> {
                $builder {
                    cur: self.cur.push_back(x),
                    rem: Default::default(),
                    phantom: Default::default(),
                }
            }

            pub fn finish(self) -> Current
            where
                Remaining: $q<$($g_lt ,)* $($g ,)* Back = Remaining>,
            {
                self.cur
            }
        }
    };
}
