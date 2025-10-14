// https://stackoverflow.com/questions/57454887/how-do-i-push_back-to-a-tuple

pub trait Tuple {
    type PushBack<T>;

    type PushFront<T>;

    fn push_back<T>(self, t: T) -> Self::PushBack<T>;

    fn push_front<T>(self, t: T) -> Self::PushFront<T>;
}

impl Tuple for () {
    type PushBack<T> = (T,);

    type PushFront<T> = (T,);

    fn push_back<T>(self, t: T) -> Self::PushBack<T> {
        (t,)
    }

    fn push_front<T>(self, t: T) -> Self::PushFront<T> {
        (t,)
    }
}

macro_rules! impl_tuple_append {
    ( () ) => {};
    ( ( $t0:ident $(, $types:ident)* ) ) => {
        impl<$t0, $($types,)*> Tuple for ($t0, $($types,)*) {
            // Trailing comma, just to be extra sure we are dealing
            // with a tuple and not a parenthesized type/expr.
            type PushBack<T> = ($t0, $($types,)* T,);

            type PushFront<T> = (T, $t0, $($types,)*);

            fn push_back<T>(self, t: T) -> Self::PushBack<T> {
                // Reuse the type identifiers to destructure ourselves:
                let ($t0, $($types,)*) = self;
                // Create a new tuple with the original elements, plus the new one:
                ($t0, $($types,)* t,)
            }

            fn push_front<T>(self, t:T) -> Self::PushFront<T> {
                let ($t0, $($types,)*) = self;
                (t, $t0, $($types,)*)
            }
        }

        // Recurse for one smaller size:
        impl_tuple_append! { ($($types),*) }
    };
}

impl_tuple_append! {
    (_1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16)
}

fn xyz() {
    type A = ();

    type B = <A as Tuple>::PushBack<i32>;
    let b = (32);

    type C = <B as Tuple>::PushBack<char>;
    let c: C = (32, 'x');

    type D = <C as Tuple>::PushFront<bool>;
    let d: D = (true, 32, 'x');

    let a = ().push_back(32).push_back('x').push_front(true);
    let d: D = a;
}
