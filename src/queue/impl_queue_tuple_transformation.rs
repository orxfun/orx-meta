#[macro_export]
macro_rules! define_queue_tuple_transformation {
    (
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
    ) => {
        define_queue_tuple_transformation!(
            lifetimes => [];
            generics => [];
            elements => [];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
        );
    };

    (
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
    ) => {
        define_queue_tuple_transformation!(
            lifetimes => [];
            generics => [];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
        );
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
    ) => {
        define_queue_tuple_transformation!(
            lifetimes => [$($g_lt)& *];
            generics => [];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
        );
    };

    (
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
    ) => {
        define_queue_tuple_transformation!(
            lifetimes => [];
            generics => [$($g:$($g_bnd$(< $( $g_bnd_g ),* >)?)| *)& *];
            elements => [$($el_bnd$(< $( $el_bnd_g ),* >)?)& *];
            queues => { trait: $q, empty: $empty, single: $single, pair: $pair };
        );
    };

    (
        lifetimes => [$($g_lt:tt)& *];
        generics => [$($g:tt:$($g_bnd:ident$(< $( $g_bnd_g:tt ),* >)?)| *)& *];
        elements => [$($el_bnd:ident$(< $( $el_bnd_g:tt ),* >)?)& *];
        queues => { trait: $q:ident, empty: $empty:ident, single: $single:ident, pair: $pair:ident };
    ) => {
        // tuple - 1

        impl<$($g_lt ,)* X1, $($g ,)*> $single<$($g_lt ,)* X1, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> X1 {
                self.f
            }
        }

        impl<$($g_lt ,)* X1, $($g ,)*> From<X1> for $single<$($g_lt ,)* X1, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: X1) -> Self {
                $single::new(x)
            }
        }

        // tuple - 2

        impl<$($g_lt ,)* X1, X2, $($g ,)*> $pair<$($g_lt ,)* X1, $single<$($g_lt ,)* X2, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2) {
                (self.f, self.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, $($g ,)*> From<(X1, X2)> for $pair<$($g_lt ,)* X1, $single<$($g_lt ,)* X2, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2)) -> Self {
                $pair::new(x.0, $single::new(x.1))
            }
        }

        // tuple - 3

        impl<$($g_lt ,)* X1, X2, X3, $($g ,)*> $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $single<$($g_lt ,)* X3, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.f, self.b.f, self.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, $($g ,)*> From<(X1, X2, X3)> for $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $single<$($g_lt ,)* X3, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $single::new(x.2)))
            }
        }

        // tuple - 4

        impl<$($g_lt ,)* X1, X2, X3, X4, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $single<$($g_lt ,)* X4, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, $($g ,)*> From<(X1, X2, X3, X4)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $single<$($g_lt ,)* X4, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $single::new(x.3))))
            }
        }

        // tuple - 5

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $single<$($g_lt ,)* X5, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, $($g ,)*> From<(X1, X2, X3, X4, X5)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $single<$($g_lt ,)* X5, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $single::new(x.4)))))
            }
        }

        // tuple - 6

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $single<$($g_lt ,)* X6, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, $($g ,)*> From<(X1, X2, X3, X4, X5, X6)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $single<$($g_lt ,)* X6, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $single::new(x.5))))))
            }
        }

        // tuple - 7

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $single<$($g_lt ,)* X7, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, $($g ,)*> From<(X1, X2, X3, X4, X5, X6, X7)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $single<$($g_lt ,)* X7, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $single::new(x.6)))))))
            }
        }

        // tuple - 8

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, X8, $($g ,)*>
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $pair<$($g_lt ,)* X7, $single<$($g_lt ,)* X8, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (self.f, self.b.f, self.b.b.f, self.b.b.b.f, self.b.b.b.b.f, self.b.b.b.b.b.f, self.b.b.b.b.b.b.f, self.b.b.b.b.b.b.b.f)
            }
        }

        impl<$($g_lt ,)* X1, X2, X3, X4, X5, X6, X7, X8, $($g ,)*> From<(X1, X2, X3, X4, X5, X6, X7, X8)> for
            $pair<$($g_lt ,)* X1, $pair<$($g_lt ,)* X2, $pair<$($g_lt ,)* X3, $pair<$($g_lt ,)* X4, $pair<$($g_lt ,)* X5, $pair<$($g_lt ,)* X6, $pair<$($g_lt ,)* X7, $single<$($g_lt ,)* X8, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>, $($g ,)*>
        where
            X1: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X2: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X3: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X4: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X5: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X6: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X7: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            X8: $( $el_bnd $( < $( $el_bnd_g ),* > )? + ) *,
            $( $g: $( $g_bnd $(<$( $g_bnd_g ),*> )? + ) * , )*
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $pair::new(x.0, $pair::new(x.1, $pair::new(x.2, $pair::new(x.3, $pair::new(x.4, $pair::new(x.5, $pair::new(x.6, $single::new(x.7))))))))
            }
        }
    };
}
