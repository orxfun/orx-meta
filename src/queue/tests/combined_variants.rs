use crate::define_queue;

#[test]
#[allow(dead_code, unused_macros)]
fn core() {
    {
        define_queue!(
            queue => [ Q, NeQ ; Empty, Single, Pair ];
        );
    }

    {
        define_queue!(
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
        );
    }

    {
        define_queue!(
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            builder => Builder;
        );
    }

    {
        define_queue!(
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
            builder => Builder;
        );
    }
}

#[test]
#[allow(dead_code, unused_macros)]
fn core_elements() {
    pub trait MyTrait {}

    {
        define_queue!(
            elements => [MyTrait | Clone];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
        );

        impl MyTrait for Empty {}
        impl<F: MyTrait + Clone> MyTrait for Single<F> {}
        impl<F: MyTrait + Clone, B: Q> MyTrait for Pair<F, B> {}
    }

    {
        define_queue!(
            elements => [MyTrait | Clone];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
        );

        impl MyTrait for Empty {}
        impl<F: MyTrait + Clone> MyTrait for Single<F> {}
        impl<F: MyTrait + Clone, B: Q> MyTrait for Pair<F, B> {}
    }

    {
        define_queue!(
            elements => [MyTrait | Clone];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            builder => Builder;
        );

        impl MyTrait for Empty {}
        impl<F: MyTrait + Clone> MyTrait for Single<F> {}
        impl<F: MyTrait + Clone, B: Q> MyTrait for Pair<F, B> {}
    }

    {
        define_queue!(
            elements => [MyTrait | Clone];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
            builder => Builder;
        );

        impl MyTrait for Empty {}
        impl<F: MyTrait + Clone> MyTrait for Single<F> {}
        impl<F: MyTrait + Clone, B: Q> MyTrait for Pair<F, B> {}
    }
}

#[test]
#[allow(dead_code, unused_macros)]
fn core_generic_elements() {
    pub trait MyTrait<'i> {}

    {
        define_queue!(
            lt => ['i];
            elements => [MyTrait<'i>];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
        );

        impl<'i> MyTrait<'i> for Empty<'i> {}
        impl<'i, F: MyTrait<'i>> MyTrait<'i> for Single<'i, F> {}
        impl<'i, F: MyTrait<'i>, B: Q<'i>> MyTrait<'i> for Pair<'i, F, B> {}
    }

    {
        define_queue!(
            lt => ['i];
            elements => [MyTrait<'i>];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
        );

        impl<'i> MyTrait<'i> for Empty<'i> {}
        impl<'i, F: MyTrait<'i>> MyTrait<'i> for Single<'i, F> {}
        impl<'i, F: MyTrait<'i>, B: Q<'i>> MyTrait<'i> for Pair<'i, F, B> {}
    }

    {
        define_queue!(
            lt => ['i];
            elements => [MyTrait<'i>];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            builder => Builder;
        );

        impl<'i> MyTrait<'i> for Empty<'i> {}
        impl<'i, F: MyTrait<'i>> MyTrait<'i> for Single<'i, F> {}
        impl<'i, F: MyTrait<'i>, B: Q<'i>> MyTrait<'i> for Pair<'i, F, B> {}
    }

    {
        define_queue!(
            lt => ['i];
            elements => [MyTrait<'i>];
            queue => [ Q, NeQ ; Empty, Single, Pair ];
            queue_of => qof;
            builder => Builder;
        );

        impl<'i> MyTrait<'i> for Empty<'i> {}
        impl<'i, F: MyTrait<'i>> MyTrait<'i> for Single<'i, F> {}
        impl<'i, F: MyTrait<'i>, B: Q<'i>> MyTrait<'i> for Pair<'i, F, B> {}
    }
}
