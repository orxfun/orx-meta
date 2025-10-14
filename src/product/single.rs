use crate::product::{empty::Empty, multi::Multi, pop::Pop, push::Push};

pub struct Single<T> {
    element: T,
}

impl<T> Single<T> {
    pub fn new(element: T) -> Self {
        Self { element }
    }

    // ref

    fn back(&self) -> &T {
        &self.element
    }

    fn front(&self) -> &T {
        &self.element
    }

    // mut

    fn back_mut(&mut self) -> &mut T {
        &mut self.element
    }

    fn front_mut(&mut self) -> &mut T {
        &mut self.element
    }
}

impl<T> Push for Single<T> {
    type PushBack<B> = Multi<T, Empty, B>;

    type PushFront<F> = Multi<F, Empty, T>;

    const LEN: usize = 1;

    fn push_back<X>(self, element: X) -> Self::PushBack<X> {
        (self.element, Empty, element).into()
    }

    fn push_front<X>(self, element: X) -> Self::PushFront<X> {
        (element, Empty, self.element).into()
    }
}

impl<T> Pop for Single<T> {
    type Front = T;

    type PopFront = Empty;

    type Back = T;

    type PopBack = Empty;

    fn pop_back(self) -> (Self::Back, Self::PopBack) {
        (self.element, Empty)
    }

    fn pop_front(self) -> (Self::Front, Self::PopFront) {
        (self.element, Empty)
    }
}
