#[macro_export]
macro_rules! define_queue_composition {
    (
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
        composition => $composition:ident;
    ) => {
        define_queue_composition!(
            lifetimes => [];
            generics => [];
            elements => [];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
            composition => $composition;
        );
    };

    (
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
        composition => $composition:ident;
    ) => {
        define_queue_composition!(
            lifetimes => [];
            generics => [];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
            composition => $composition;
        );
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
        composition => $composition:ident;
    ) => {
        define_queue_composition!(
            lifetimes => [$($g_lt)& *];
            generics => [];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
            composition => $composition;
        );
    };

    (
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
        composition => $composition:ident;
    ) => {
        define_queue_composition!(
            lifetimes => [];
            generics => [$($g:$($g_bnd$(< $( $g_bnd_g ),* >)?)| *)& *];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
            composition => $composition;
        );
    };

    // complete impl
    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
        composition => $composition:ident;
    ) => {
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
                q.push(x)
            }
        }
    };
}
