#[macro_export]
macro_rules! impl_meta_queue {
    // with item bounds
    (
        [$($item_bounds:ident),*],
        $never:ident,
        $queue_trait:ident,
        $non_empty_queue_trait:ident,
        $empty:ident,
        $single:ident,
        $pair:ident
    ) => {

        // traits
        pub trait $queue_trait {
            type PushBack<X>: $non_empty_queue_trait
            where
                X: $( $item_bounds + )*;

               type Front: $( $item_bounds + )*;

               type Back: $queue_trait;

               fn push_back<X>(self, x: X) -> Self::PushBack<X>
               where
                   X: $( $item_bounds + )*;

               fn len(&self) -> usize;

               fn is_empty(&self) -> bool {
                   self.len() == 0
               }
        }

           pub trait $non_empty_queue_trait: $queue_trait {
               fn front(&self) -> &Self::Front;

               fn pop_front(self) -> (Self::Front, Self::Back);
           }

           // empty

           #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
           pub struct $empty;

           impl $queue_trait for $empty {
               type PushBack<X>
                   = $single<X>
               where
                   X: $( $item_bounds + )*;

               type Front = $never;

               type Back = $empty;

               fn push_back<X>(self, x: X) -> Self::PushBack<X>
               where
                   X: $( $item_bounds + )*,
               {
                   $single(x)
               }

               fn len(&self) -> usize {
                   0
               }
           }

           // single

           #[derive(Debug, Clone, Copy)]
           pub struct $single<F>(F)
           where
               F: $( $item_bounds + )*;

           impl<F> $queue_trait for $single<F>
           where
               F: $( $item_bounds + )*,
           {
               type PushBack<X>
                   = $pair<F, $single<X>>
               where
                   X: $( $item_bounds + )*;

               type Front = F;

               type Back = $empty;

               fn push_back<X>(self, x: X) -> Self::PushBack<X>
               where
                   X: $( $item_bounds + )*,
               {
                   $pair(self.0, $single(x))
               }

               fn len(&self) -> usize {
                   1
               }
           }

           impl<F> $non_empty_queue_trait for $single<F>
           where
               F: $( $item_bounds + )*,
           {
               fn front(&self) -> &Self::Front {
                   &self.0
               }

               fn pop_front(self) -> (Self::Front, Self::Back) {
                   (self.0, $empty)
               }
           }

           // pair

           #[derive(Debug, Clone, Copy)]
           pub struct $pair<F, B>(F, B)
           where
               F: $( $item_bounds + )*,
               B: $queue_trait;

           impl<F, B> $queue_trait for $pair<F, B>
           where
               F: $( $item_bounds + )*,
               B: $queue_trait,
           {
               type PushBack<X>
                   = $pair<F, B::PushBack<X>>
               where
                   X: $( $item_bounds + )*;

               type Front = F;

               type Back = B;

               fn push_back<X>(self, x: X) -> Self::PushBack<X>
               where
                   X: $( $item_bounds + )*,
               {
                   $pair(self.0, self.1.push_back(x))
               }

               fn len(&self) -> usize {
                   1 + self.1.len()
               }
           }

           impl<F, B> $non_empty_queue_trait for $pair<F, B>
           where
               F: $( $item_bounds + )*,
               B: $queue_trait,
           {
               fn front(&self) -> &Self::Front {
                   &self.0
               }

               fn pop_front(self) -> (Self::Front, Self::Back) {
                   (self.0, self.1)
               }
           }
    };

    // with lifetimes
    (
        $lt:lifetime,
        $item_bound:ident,
        $never:ident,
        $queue_trait:ident,
        $non_empty_queue_trait:ident,
        $empty:ident,
        $single:ident,
        $pair:ident
    ) => {

        // traits
        pub trait $queue_trait<$lt> {
            type PushBack<X>: $non_empty_queue_trait<$lt>
            where
                X: $item_bound<$lt>;

               type Front: $item_bound<$lt>;

               type Back: $queue_trait<$lt>;

               fn push_back<X>(self, x: X) -> Self::PushBack<X>
               where
                   X: $item_bound<$lt>;

               fn len(&self) -> usize;

               fn is_empty(&self) -> bool {
                   self.len() == 0
               }
        }

        pub trait $non_empty_queue_trait<$lt>: $queue_trait<$lt> {
            fn front(&self) -> &Self::Front;

            fn pop_front(self) -> (Self::Front, Self::Back);
        }

        // empty

        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $empty;

        impl<$lt> $queue_trait<$lt> for $empty {
            type PushBack<X>
                = $single<$lt, X>
            where
                X: $item_bound<$lt>;

            type Front = $never;

            type Back = $empty;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $item_bound<$lt>,
            {
                $single(x, core::marker::PhantomData)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // single

        #[derive(Debug, Clone, Copy)]
        pub struct $single<$lt, F>(F, core::marker::PhantomData<&$lt ()>)
        where
            F: $item_bound<$lt>;

        impl<$lt, F> $queue_trait<$lt> for $single<$lt, F>
        where
            F: $item_bound<$lt>,
        {
            type PushBack<X>
                = $pair<$lt, F, $single<$lt, X>>
            where
                X: $item_bound<$lt>;

            type Front = F;

            type Back = $empty;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $item_bound<$lt>,
            {
                $pair(self.0, $single(x, core::marker::PhantomData), core::marker::PhantomData)
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<$lt, F> $non_empty_queue_trait<$lt> for $single<$lt, F>
        where
            F: $item_bound<$lt>,
        {
            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, $empty)
            }
        }

        // pair

        #[derive(Debug, Clone, Copy)]
        pub struct $pair<$lt, F, B>(F, B, core::marker::PhantomData<&$lt ()>)
        where
            F: $item_bound<$lt>,
            B: $queue_trait<$lt>;

        impl<$lt, F, B> $queue_trait<$lt> for $pair<$lt, F, B>
        where
            F: $item_bound<$lt>,
            B: $queue_trait<$lt>,
        {
            type PushBack<X>
                = $pair<$lt, F, B::PushBack<X>>
            where
                X: $item_bound<$lt>;

            type Front = F;

            type Back = B;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>
            where
                X: $item_bound<$lt>,
            {
                $pair(self.0, self.1.push_back(x), core::marker::PhantomData)
            }

            fn len(&self) -> usize {
                1 + self.1.len()
            }
        }

        impl<$lt, F, B> $non_empty_queue_trait<$lt> for $pair<$lt, F, B>
        where
            F: $item_bound<$lt>,
            B: $queue_trait<$lt>,
        {
            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, self.1)
            }
        }
    };
}

pub trait Xyz {}
pub trait Www {}
pub trait Ooo {}

#[derive(Clone, Copy)]
pub enum Never {}
impl Xyz for Never {}
impl Www for Never {}
impl Ooo for Never {}

impl_meta_queue!(
    [Xyz, Www],
    Never,
    Queue2,
    NonEmptyQueue2,
    Empty2,
    Single2,
    Pair2
);

pub trait Xox<'i> {}
pub trait Yoyo<'j> {}
impl<'i> Xox<'i> for Never {}
impl<'i> Yoyo<'i> for Never {}

impl_meta_queue!(
    'i,
    Xox,
    Never,
    Queue3,
    NonEmptyQueue3,
    Empty3,
    Single3,
    Pair3
);
