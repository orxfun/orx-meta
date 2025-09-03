#[macro_export]
macro_rules! define_queue_builder {
    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        queue => $q:ident;
        empty_queue => $empty:ident;
        builder => $builder:ident;
    ) => {
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
    };
}

use crate::define_queue;

define_queue!(
    names => {
        traits: {
            queue: Queue,
            non_empty_queue: NonEmptyQueue,
        },
        structs: {
            empty: Empty,
            single: Single,
            pair: Pair,
            composition: QueueComposition,
            builder: Builder,
        },
    };
);

define_queue_builder!(
    lifetimes => [];
    generics => [];
    queue => Queue;
    empty_queue => Empty;
    builder => Blocker;
);
