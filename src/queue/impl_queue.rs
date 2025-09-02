#[macro_export]
macro_rules! define_queue_old {
    // no bounds
    (
        $trait_queue:ident,
        $trait_non_empty_queue:ident,
        $empty:ident,
        $single:ident,
        $pair:ident,
        $composition:ident,
        $never:ident,
        $builder:ident
    ) => {
        // traits

        pub trait $trait_queue {
            type PushBack<X>: $trait_non_empty_queue;

            type Front;

            type Back: $trait_queue;

            fn push_back<X>(self, x: X) -> Self::PushBack<X>;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        pub trait $trait_non_empty_queue: $trait_queue {
            fn into_front(self) -> Self::Front;

            fn pop_front(self) -> (Self::Front, Self::Back);

            fn front(&self) -> &Self::Front;

            fn front_back(&self) -> (&Self::Front, &Self::Back);
        }

        // impl: empty

        #[derive(Clone, Copy, Debug, Default)]
        pub struct $empty;

        impl $trait_queue for $empty {
            type PushBack<X> = $single<X>;

            type Front = $never;

            type Back = Self;

            fn push_back<X>(self, x: X) -> Self::PushBack<X> {
                $single(x)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // impl: single

        #[derive(Clone, Copy, Debug)]
        pub struct $single<F>(F);

        impl<F> $trait_queue for $single<F> {
            type PushBack<X> = $pair<F, $single<X>>;

            type Front = F;

            type Back = $empty;

            fn push_back<X>(self, x: X) -> Self::PushBack<X> {
                $pair(self.0, $single(x))
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<F> $trait_non_empty_queue for $single<F> {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, $empty)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &$empty)
            }
        }

        // impl: pair

        #[derive(Clone, Copy, Debug)]
        pub struct $pair<F, B: $trait_queue>(F, B);

        impl<F, B: $trait_queue> $trait_queue for $pair<F, B> {
            type PushBack<X> = $pair<F, B::PushBack<X>>;

            type Front = F;

            type Back = B;

            fn push_back<X>(self, x: X) -> Self::PushBack<X> {
                $pair(self.0, self.1.push_back(x))
            }

            fn len(&self) -> usize {
                1 + self.1.len()
            }
        }

        impl<F, B: $trait_queue> $trait_non_empty_queue for $pair<F, B> {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, self.1)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &self.1)
            }
        }

        // composition

        pub struct $composition;

        impl $composition {
            pub fn empty() -> $empty {
                $empty
            }

            pub fn single<X>(x: X) -> $single<X> {
                $single(x)
            }

            pub fn compose<C: $trait_queue, X>(q: C, x: X) -> C::PushBack<X> {
                q.push_back(x)
            }
        }

        // builder

        pub struct $builder<Rem, Cur>(Cur, core::marker::PhantomData<Rem>)
        where
            Rem: $trait_queue,
            Cur: $trait_queue;

        impl<Rem> $builder<Rem, $empty>
        where
            Rem: $trait_queue,
        {
            pub fn new() -> Self {
                Self($empty, core::marker::PhantomData)
            }
        }

        impl<Rem, Cur> $builder<Rem, Cur>
        where
            Rem: $trait_queue,
            Cur: $trait_queue,
        {
            pub fn push_back(
                self,
                x: Rem::Front,
            ) -> $builder<Rem::Back, Cur::PushBack<Rem::Front>> {
                let current = self.0.push_back(x);
                $builder(current, core::marker::PhantomData)
            }

            pub fn finish(self) -> Cur
            where
                Rem: $trait_queue<Back = Rem>,
            {
                self.0
            }
        }

        // tuple support - 1

        impl<X1> $single<X1> {
            pub fn into_tuple(self) -> X1 {
                self.0
            }
        }

        impl<X1> From<X1> for $single<X1> {
            fn from(x: X1) -> Self {
                $single(x)
            }
        }

        // tuple support - 2

        impl<X1, X2> $pair<X1, $single<X2>> {
            pub fn into_tuple(self) -> (X1, X2) {
                (self.0, self.1.0)
            }
        }

        impl<X1, X2> From<(X1, X2)> for $pair<X1, $single<X2>> {
            fn from(x: (X1, X2)) -> Self {
                $single::from(x.0).push_back(x.1)
            }
        }

        // tuple support - 3

        impl<X1, X2, X3> $pair<X1, $pair<X2, $single<X3>>> {
            pub fn into_tuple(self) -> (X1, X2, X3) {
                (self.0, self.1.0, self.1.1.0)
            }
        }

        impl<X1, X2, X3> From<(X1, X2, X3)> for $pair<X1, $pair<X2, $single<X3>>> {
            fn from(x: (X1, X2, X3)) -> Self {
                $single::from(x.0).push_back(x.1).push_back(x.2)
            }
        }

        // tuple support - 4

        impl<X1, X2, X3, X4> $pair<X1, $pair<X2, $pair<X3, $single<X4>>>> {
            pub fn into_tuple(self) -> (X1, X2, X3, X4) {
                (self.0, self.1.0, self.1.1.0, self.1.1.1.0)
            }
        }

        impl<X1, X2, X3, X4> From<(X1, X2, X3, X4)> for $pair<X1, $pair<X2, $pair<X3, $single<X4>>>> {
            fn from(x: (X1, X2, X3, X4)) -> Self {
                $single::from(x.0)
                    .push_back(x.1)
                    .push_back(x.2)
                    .push_back(x.3)
            }
        }

        // tuple support - 5

        impl<X1, X2, X3, X4, X5> $pair<X1, $pair<X2, $pair<X3, $pair<X4, $single<X5>>>>> {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
                (self.0, self.1.0, self.1.1.0, self.1.1.1.0, self.1.1.1.1.0)
            }
        }

        impl<X1, X2, X3, X4, X5> From<(X1, X2, X3, X4, X5)>
            for $pair<X1, $pair<X2, $pair<X3, $pair<X4, $single<X5>>>>>
        {
            fn from(x: (X1, X2, X3, X4, X5)) -> Self {
                $single::from(x.0)
                    .push_back(x.1)
                    .push_back(x.2)
                    .push_back(x.3)
                    .push_back(x.4)
            }
        }

        // tuple support - 6

        impl<X1, X2, X3, X4, X5, X6> $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $single<X6>>>>>> {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
                (
                    self.0,
                    self.1.0,
                    self.1.1.0,
                    self.1.1.1.0,
                    self.1.1.1.1.0,
                    self.1.1.1.1.1.0,
                )
            }
        }

        impl<X1, X2, X3, X4, X5, X6> From<(X1, X2, X3, X4, X5, X6)>
            for $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $single<X6>>>>>>
        {
            fn from(x: (X1, X2, X3, X4, X5, X6)) -> Self {
                $single::from(x.0)
                    .push_back(x.1)
                    .push_back(x.2)
                    .push_back(x.3)
                    .push_back(x.4)
                    .push_back(x.5)
            }
        }

        // tuple support - 7

        impl<X1, X2, X3, X4, X5, X6, X7>
            $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $pair<X6, $single<X7>>>>>>>
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
                (
                    self.0,
                    self.1.0,
                    self.1.1.0,
                    self.1.1.1.0,
                    self.1.1.1.1.0,
                    self.1.1.1.1.1.0,
                    self.1.1.1.1.1.1.0,
                )
            }
        }

        impl<X1, X2, X3, X4, X5, X6, X7> From<(X1, X2, X3, X4, X5, X6, X7)>
            for $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $pair<X6, $single<X7>>>>>>>
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7)) -> Self {
                $single::from(x.0)
                    .push_back(x.1)
                    .push_back(x.2)
                    .push_back(x.3)
                    .push_back(x.4)
                    .push_back(x.5)
                    .push_back(x.6)
            }
        }

        // tuple support - 8

        impl<X1, X2, X3, X4, X5, X6, X7, X8>
            $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $pair<X6, $pair<X7, $single<X8>>>>>>>>
        {
            pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
                (
                    self.0,
                    self.1.0,
                    self.1.1.0,
                    self.1.1.1.0,
                    self.1.1.1.1.0,
                    self.1.1.1.1.1.0,
                    self.1.1.1.1.1.1.0,
                    self.1.1.1.1.1.1.1.0,
                )
            }
        }

        impl<X1, X2, X3, X4, X5, X6, X7, X8> From<(X1, X2, X3, X4, X5, X6, X7, X8)>
            for $pair<X1, $pair<X2, $pair<X3, $pair<X4, $pair<X5, $pair<X6, $pair<X7, $single<X8>>>>>>>>
        {
            fn from(x: (X1, X2, X3, X4, X5, X6, X7, X8)) -> Self {
                $single::from(x.0)
                    .push_back(x.1)
                    .push_back(x.2)
                    .push_back(x.3)
                    .push_back(x.4)
                    .push_back(x.5)
                    .push_back(x.6)
                    .push_back(x.7)
            }
        }
    };

    // with bounds
    (
        $trait_queue:ident,
        $trait_non_empty_queue:ident,
        $empty:ident,
        $single:ident,
        $pair:ident,
        $composition:ident,
        $never:ident,
        $builder:ident,
        $req:ident
    ) => {
        // traits

        pub trait $trait_queue {
            type PushBack<X: $req>: $trait_non_empty_queue;

            type Front: $req;

            type Back: $trait_queue;

            fn push_back<X: $req>(self, x: X) -> Self::PushBack<X>;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        pub trait $trait_non_empty_queue: $trait_queue {
            fn into_front(self) -> Self::Front;

            fn pop_front(self) -> (Self::Front, Self::Back);

            fn front(&self) -> &Self::Front;

            fn front_back(&self) -> (&Self::Front, &Self::Back);
        }

        // impl: empty

        #[derive(Clone, Copy, Debug, Default)]
        pub struct $empty;

        impl $trait_queue for $empty {
            type PushBack<X: $req> = $single<X>;

            type Front = $never;

            type Back = Self;

            fn push_back<X: $req>(self, x: X) -> Self::PushBack<X> {
                $single(x)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // impl: single

        #[derive(Clone, Copy, Debug)]
        pub struct $single<F: $req>(F);

        impl<F: $req> $trait_queue for $single<F> {
            type PushBack<X: $req> = $pair<F, $single<X>>;

            type Front = F;

            type Back = $empty;

            fn push_back<X: $req>(self, x: X) -> Self::PushBack<X> {
                $pair(self.0, $single(x))
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<F: $req> $trait_non_empty_queue for $single<F> {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, $empty)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &$empty)
            }
        }

        // impl: pair

        #[derive(Clone, Copy, Debug)]
        pub struct $pair<F: $req, B: $trait_queue>(F, B);

        impl<F: $req, B: $trait_queue> $trait_queue for $pair<F, B> {
            type PushBack<X: $req> = $pair<F, B::PushBack<X>>;

            type Front = F;

            type Back = B;

            fn push_back<X: $req>(self, x: X) -> Self::PushBack<X> {
                $pair(self.0, self.1.push_back(x))
            }

            fn len(&self) -> usize {
                1 + self.1.len()
            }
        }

        impl<F: $req, B: $trait_queue> $trait_non_empty_queue for $pair<F, B> {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, self.1)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &self.1)
            }
        }

        // composition

        pub struct $composition;

        impl $composition {
            pub fn empty() -> $empty {
                $empty
            }

            pub fn single<X: $req>(x: X) -> $single<X> {
                $single(x)
            }

            pub fn compose<C: $trait_queue, X: $req>(q: C, x: X) -> C::PushBack<X> {
                q.push_back(x)
            }
        }

        // builder

        pub struct $builder<Rem, Cur>(Cur, core::marker::PhantomData<Rem>)
        where
            Rem: $trait_queue,
            Cur: $trait_queue;

        impl<Rem> $builder<Rem, $empty>
        where
            Rem: $trait_queue,
        {
            pub fn new() -> Self {
                Self($empty, core::marker::PhantomData)
            }
        }

        impl<Rem, Cur> $builder<Rem, Cur>
        where
            Rem: $trait_queue,
            Cur: $trait_queue,
        {
            pub fn push_back(
                self,
                x: Rem::Front,
            ) -> $builder<Rem::Back, Cur::PushBack<Rem::Front>> {
                let current = self.0.push_back(x);
                $builder(current, core::marker::PhantomData)
            }

            pub fn finish(self) -> Cur
            where
                Rem: $trait_queue<Back = Rem>,
            {
                self.0
            }
        }
    };

    // with lt bounds
    (
        $trait_queue:ident,
        $trait_non_empty_queue:ident,
        $empty:ident,
        $single:ident,
        $pair:ident,
        $composition:ident,
        $never:ident,
        $builder:ident,
        $req:ident,
        $lt:lifetime
    ) => {
        // traits

        pub trait $trait_queue<$lt> {
            type PushBack<X: $req<$lt>>: $trait_non_empty_queue<$lt>;

            type Front: $req<$lt>;

            type Back: $trait_queue<$lt>;

            fn push_back<X: $req<$lt>>(self, x: X) -> Self::PushBack<X>;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        pub trait $trait_non_empty_queue<$lt>: $trait_queue<$lt> {
            fn into_front(self) -> Self::Front;

            fn pop_front(self) -> (Self::Front, Self::Back);

            fn front(&self) -> &Self::Front;

            fn front_back(&self) -> (&Self::Front, &Self::Back);
        }

        // impl: empty

        #[derive(Clone, Copy, Debug, Default)]
        pub struct $empty;

        impl<$lt> $trait_queue<$lt> for $empty {
            type PushBack<X: $req<$lt>> = $single<$lt, X>;

            type Front = $never;

            type Back = Self;

            fn push_back<X: $req<$lt>>(self, x: X) -> Self::PushBack<X> {
                $single(x, core::marker::PhantomData)
            }

            fn len(&self) -> usize {
                0
            }
        }

        // impl: single

        #[derive(Clone, Copy, Debug)]
        pub struct $single<$lt, F: $req<$lt>>(F, core::marker::PhantomData<&$lt()>);

        impl<$lt, F: $req<$lt>> $trait_queue<$lt> for $single<$lt, F> {
            type PushBack<X: $req<$lt>> = $pair<$lt, F, $single<$lt, X>>;

            type Front = F;

            type Back = $empty;

            fn push_back<X: $req<$lt>>(self, x: X) -> Self::PushBack<X> {
                $pair(
                    self.0,
                    $single(x, core::marker::PhantomData),
                    core::marker::PhantomData,
                )
            }

            fn len(&self) -> usize {
                1
            }
        }

        impl<$lt, F: $req<$lt>> $trait_non_empty_queue<$lt> for $single<$lt, F> {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, $empty)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &$empty)
            }
        }

        // impl: pair

        #[derive(Clone, Copy, Debug)]
        pub struct $pair<$lt, F: $req<$lt>, B: $trait_queue<$lt>>(
            F,
            B,
            core::marker::PhantomData<&$lt()>,
        );

        impl<$lt, F: $req<$lt>, B: $trait_queue<$lt>> $trait_queue<$lt> for $pair<$lt, F, B> {
            type PushBack<X: $req<$lt>> = $pair<$lt, F, B::PushBack<X>>;

            type Front = F;

            type Back = B;

            fn push_back<X: $req<$lt>>(self, x: X) -> Self::PushBack<X> {
                $pair(self.0, self.1.push_back(x), core::marker::PhantomData)
            }

            fn len(&self) -> usize {
                1 + self.1.len()
            }
        }

        impl<$lt, F: $req<$lt>, B: $trait_queue<$lt>> $trait_non_empty_queue<$lt>
            for $pair<$lt, F, B>
        {
            fn into_front(self) -> Self::Front {
                self.0
            }

            fn pop_front(self) -> (Self::Front, Self::Back) {
                (self.0, self.1)
            }

            fn front(&self) -> &Self::Front {
                &self.0
            }

            fn front_back(&self) -> (&Self::Front, &Self::Back) {
                (&self.0, &self.1)
            }
        }

        // composition

        pub struct $composition;

        impl $composition {
            pub fn empty() -> $empty {
                $empty
            }

            pub fn single<$lt, X: $req<$lt>>(x: X) -> $single<$lt, X> {
                $single(x, core::marker::PhantomData)
            }

            pub fn compose<$lt, C: $trait_queue<$lt>, X: $req<$lt>>(q: C, x: X) -> C::PushBack<X> {
                q.push_back(x)
            }
        }

        // builder

        pub struct $builder<$lt, Rem, Cur>(Cur, core::marker::PhantomData<&$lt Rem>)
        where
            Rem: $trait_queue<$lt>,
            Cur: $trait_queue<$lt>;

        impl<$lt, Rem> $builder<$lt, Rem, $empty>
        where
            Rem: $trait_queue<$lt>,
        {
            pub fn new() -> Self {
                Self($empty, core::marker::PhantomData)
            }
        }

        impl<$lt, Rem, Cur> $builder<$lt, Rem, Cur>
        where
            Rem: $trait_queue<$lt>,
            Cur: $trait_queue<$lt>,
        {
            pub fn push_back(
                self,
                x: Rem::Front,
            ) -> $builder<$lt, Rem::Back, Cur::PushBack<Rem::Front>> {
                let current = self.0.push_back(x);
                $builder(current, core::marker::PhantomData)
            }

            pub fn finish(self) -> Cur
            where
                Rem: $trait_queue<$lt, Back = Rem>,
            {
                self.0
            }
        }
    };
}
