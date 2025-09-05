#[macro_export]
macro_rules! define_queue {
    (
        lt => [$($g_lt:tt), *];
        generics => [ $( $g:tt $( : $( $g_bnd:ident $( < $( $g_bnd_g:tt ),* > )? )| * )? ), * ];
        elements => [ $( $el_bnd:ident $( < $( $el_bnd_g:tt ),* > )? )| * ];
        queue => [$q:ident, $q_ne:ident ; $empty:ident, $single:ident, $pair:ident];

        queue_of => $queue_of:ident;
        builder => $builder:ident;
    ) => {
        $crate::define_queue_core!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q, $q_ne ; $empty, $single, $pair];
        );
        $crate::define_queue_of!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q, $q_ne ; $empty, $single, $pair];
            queue_of => $queue_of;
        );

        $crate::define_queue_builder!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            queue => [$q, $q_ne ; $empty, $single, $pair];
            builder => $builder;
        );

        $crate::define_queue_tuple_transformation!(
            lt => [$($g_lt), *];
            generics => [ $( $g $( : $( $g_bnd $( < $( $g_bnd_g ),* > )? )| * )? ), * ];
            elements => [ $( $el_bnd $( < $( $el_bnd_g ),* > )? )| * ];
            queue => [$q, $q_ne ; $empty, $single, $pair];
        );
    };
}
