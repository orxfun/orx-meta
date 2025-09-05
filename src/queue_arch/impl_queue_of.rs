#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

// $pair<$($g_lt ,)* $($g ,)* F, B>

#[macro_export]
macro_rules! define_queue_of_zzz {
    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
        queue_of => $queue_of:ident;
    ) => {
        macro_rules! $queue_of {
            () => { $empty<$($g_lt ,)* $($g ,)*> };
            ($t1:ty) => { $single<$($g_lt ,)* $t1, $($g ,)*> };
            ($t1:ty, $t2:ty) => { $pair<$($g_lt ,)* $($g ,)* $t1, $single<$($g_lt ,)* $t2, $($g ,)*>> };
            ($t1:ty, $t2:ty, $t3:ty) => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $single<$($g_lt ,)* $t3, $($g ,)*>>> };
            ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $single<$($g_lt ,)* $t4, $($g ,)*>>>> };
            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty)
                => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $pair<$($g_lt ,)* $($g ,)* $t4, $single<$($g_lt ,)* $t5, $($g ,)*>>>>> };
            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty)
                => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $pair<$($g_lt ,)* $($g ,)* $t4, $pair<$($g_lt ,)* $($g ,)* $t5, $single<$($g_lt ,)* %t6, $($g ,)*>>>>>> };
            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty)
                => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $pair<$($g_lt ,)* $($g ,)* $t4, $pair<$($g_lt ,)* $($g ,)* $t5, $pair<$($g_lt ,)* $($g ,)* $t6, $single<$($g_lt ,)* $t7, $($g ,)*>>>>>>> };
            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty)
                => { $pair<$($g_lt ,)* $($g ,)* $t1, $pair<$($g_lt ,)* $($g ,)* $t2, $pair<$($g_lt ,)* $($g ,)* $t3, $pair<$($g_lt ,)* $($g ,)* $t4, $pair<$($g_lt ,)* $($g ,)* $t5, $pair<$($g_lt ,)* $($g ,)* $t6, $pair<$($g_lt ,)* $($g ,)* $t7, $single<$($g_lt ,)* $t8, $($g ,)*>>>>>>>> };
        }

        // crate::with_dollar_sign! {
        //     ($d:tt) => {
        //         macro_rules! $queue_of {
        //             ([$d($qg:tt)& *]) => { $empty<$d($qg),*> };
        //             ([$d($qg:tt)& *], $t1:ty) => { $single<$d($qg),*, $t1> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty) => { $pair<$d($qg),*, $t1, $single<$d($qg),*, $t2>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty) => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $single<$d($qg),*, $t3>>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty, $t4:ty) => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $single<$d($qg),*, $t4>>>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty)
        //                 => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $single<$d($qg),*, $t5>>>>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty)
        //                 => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $single<$d($qg),*, $t6>>>>>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty)
        //                 => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $pair<$d($qg),*, $t6, $single<$d($qg),*, $t7>>>>>>> };
        //             ([$d($qg:tt)& *], $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty)
        //                 => { $pair<$d($qg),*, $t1, $pair<$d($qg),*, $t2, $pair<$d($qg),*, $t3, $pair<$d($qg),*, $t4, $pair<$d($qg),*, $t5, $pair<$d($qg),*, $t6, $pair<$d($qg),*, $t7, $single<$d($qg),*, $t8>>>>>>>> };

        //             () => { $empty };
        //             ($t1:ty) => { $single<$t1> };
        //             ($t1:ty, $t2:ty) => { $pair<$t1, $single<$t2>> };
        //             ($t1:ty, $t2:ty, $t3:ty) => { $pair<$t1, $pair<$t2, $single<$t3>>> };
        //             ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => { $pair<$t1, $pair<$t2, $pair<$t3, $single<$t4>>>> };
        //             ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty)
        //                 => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $single<$t5>>>>> };
        //             ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty)
        //                 => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $single<$t6>>>>>> };
        //             ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty)
        //                 => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $pair<$t6, $single<$t7>>>>>>> };
        //             ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty)
        //                 => { $pair<$t1, $pair<$t2, $pair<$t3, $pair<$t4, $pair<$t5, $pair<$t6, $pair<$t7, $single<$t8>>>>>>>> };
        //         }
        //     };
        // }
    };
}
