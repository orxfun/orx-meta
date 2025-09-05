#![allow(dead_code)]

#[macro_export]
macro_rules! define_queue {
    // q
    (
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [];generics => [];elements => [];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [$($g_lt)& *];generics => [];elements => [];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [];generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];elements => [];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [];generics => [];elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [$($g_lt)& *];generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];elements => [];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [$($g_lt)& *];generics => [];elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [];generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [$($g_lt)& *];generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
    };

    // q & q-of

    (
        lifetimes => [$($g_lt:tt)& *];
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
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queue => {traits: $q:ident, $q_ne:ident; structs: $empty:ident, $single:ident, $pair:ident;};
        queue_of => $queue_of:ident;
    ) => {
        crate::define_queue_core_zzz!(lifetimes => [$($g_lt)& *];generics => [$($g:$($g_bnd$(< $( $g_bnd_g ),* >)?)| *)& *];elements => [$( $el_bnd $( < $( $el_bnd_g ),* > )?)& * ];names => { traits: { queue: $q, non_empty_queue: $q_ne}, structs: { empty: $empty, single: $single, pair: $pair}};);
        crate::define_queue_of!(lifetimes => [$($g_lt)& *];generics => [$( $g: $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )& * ];queue => {traits: $q, $q_ne; structs: $empty, $single, $pair;};queue_of => $queue_of;);
    };
}

// define_queue!(
//     lifetimes => [];
//     generics => [T: & Q:];
//     elements => [];
//     queue => { traits: Q, NeQ; structs: Em, S, P; };
//     // queue_of => qof;
// );

// type X<'a, T> = qof!(usize, char, u32);

crate::define_queue_core_zzz!(
    lifetimes => [];
    generics => [];
    elements => [];
    names => {
        traits: { queue: InQueue, non_empty_queue: InNonEmptyQueue },
        structs: { empty: InEmpty, single: InSingle, pair: InPair }
    };
);

//          generics => [
//             $(
//                 $g:tt:
//                 $(
//                     $g_bnd:ident
//                     $(
//                         < $( $g_bnd_g:tt ),* >
//                     )?
//                 )| *
//             )& *
//         ];

// generics => [$($g:$($g_bnd$(< $( $g_bnd_g ),* >)?)| *)& *];
