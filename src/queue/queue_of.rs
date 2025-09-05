#![allow(dead_code)]

use crate::define_queue_core;

#[macro_export]
macro_rules! define_queue_of {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];
        queue_of => $queue_of:ident;
    ) => {
        macro_rules! $queue_of {
            () => {
                $empty<$($g_lt ,)* $($g ,)*>
            };

            ($t1:ty) => {
                $single<$($g_lt ,)* $($g ,)* $t1>
            };

            ($t1:ty, $t2:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1, $single<$($g_lt ,)* $($g ,)* $t2>>
            };

            ($t1:ty, $t2:ty, $t3:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2, $single<$($g_lt ,)* $($g ,)* $t3>>
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3, $single<$($g_lt ,)* $($g ,)* $t4>>
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4, $single<$($g_lt ,)* $($g ,)* $t5>>
                        >
                    >
                >
            };

            ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
                $pair<$($g_lt ,)* $($g ,)* $t1,
                    $pair<$($g_lt ,)* $($g ,)* $t2,
                        $pair<$($g_lt ,)* $($g ,)* $t3,
                            $pair<$($g_lt ,)* $($g ,)* $t4,
                                $pair<$($g_lt ,)* $($g ,)* $t5, $single<$($g_lt ,)* $($g ,)* $t6>>
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
                                    $pair<$($g_lt ,)* $($g ,)* $t6, $single<$($g_lt ,)* $($g ,)* $t7>>
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
                                        $pair<$($g_lt ,)* $($g ,)* $t7, $single<$($g_lt ,)* $($g ,)* $t8>>
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
                                            $pair<$($g_lt ,)* $($g ,)* $t8, $single<$($g_lt ,)* $($g ,)* $t9>>
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
                                                $pair<$($g_lt ,)* $($g ,)* $t9, $single<$($g_lt ,)* $($g ,)* $t10>>
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
                                                    $pair<$($g_lt ,)* $($g ,)* $t10, $single<$($g_lt ,)* $($g ,)* $t11>>
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
                                                        $pair<$($g_lt ,)* $($g ,)* $t11, $single<$($g_lt ,)* $($g ,)* $t12>>
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
                                                            $pair<$($g_lt ,)* $($g ,)* $t12, $single<$($g_lt ,)* $($g ,)* $t13>>
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
                                                                $pair<$($g_lt ,)* $($g ,)* $t13, $single<$($g_lt ,)* $($g ,)* $t14>>
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
                                                                    $pair<$($g_lt ,)* $($g ,)* $t14, $single<$($g_lt ,)* $($g ,)* $t15>>
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
                                                                        $pair<$($g_lt ,)* $($g ,)* $t15, $single<$($g_lt ,)* $($g ,)* $t16>>
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

pub trait Xof<Y> {}
pub trait Yoz<'a, S> {}

pub trait Req {}
define_queue_core!(
    lt => ['a];
    generics => [T];
    elements => [Req];
    queue => [ Q, NeQ ; Em, Sng, Pr ];
);
impl<'a, T> Req for Em<'a, T> {}
impl<'a, T, F: Req> Req for Sng<'a, T, F> {}
impl<'a, T, F: Req, B: Q<'a, T>> Req for Pr<'a, T, F, B> {}

define_queue_of!(
    lt => ['a];
    generics => [T];
    queue => [ Q, NeQ ; Em, Sng, Pr ];
    queue_of => q;
);

type X0<'a, T> = q!();
type X1<'a, T> = q!(char);
type X2<'a, T> = q!(char, &'a [T]);
type X3<'a, T> = q!(
    char,
    &'a [T],
    bool,
    u8,
    u16,
    u32,
    u64,
    i8,
    bool,
    char,
    u32,
    i64,
    usize
);
