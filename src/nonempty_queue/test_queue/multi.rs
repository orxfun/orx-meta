#![allow(dead_code)]
use crate::nonempty_queue::test_queue::{QueueSingle, StQueue};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Queue<F, B>
where
    B: StQueue,
{
    front: F,
    back: B,
}

impl<F, B> StQueue for Queue<F, B>
where
    B: StQueue,
{
    type PushBack<Elem> = Queue<F, B::PushBack<Elem>>;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    #[inline(always)]
    fn front(&self) -> &F {
        &self.front
    }

    #[inline(always)]
    fn front_mut(&mut self) -> &mut F {
        &mut self.front
    }

    #[inline(always)]
    fn into_front(self) -> F {
        self.front
    }

    #[inline(always)]
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem> {
        Queue::from_fb(self.front, self.back.push(element))
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    pub(super) fn from_fb(front: F, back: B) -> Self {
        Self { front, back }
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    // ref

    #[inline(always)]
    pub fn back(&self) -> &B {
        &self.back
    }

    // mut

    #[inline(always)]
    pub fn back_mut(&mut self) -> &mut B {
        &mut self.back
    }

    #[inline(always)]
    pub fn front_back_mut(&mut self) -> (&mut F, &mut B) {
        (&mut self.front, &mut self.back)
    }

    // into

    #[inline(always)]
    pub fn into_back(self) -> B {
        self.back
    }

    #[inline(always)]
    pub fn pop(self) -> (F, B) {
        (self.front, self.back)
    }
}

// tuple

type S<F> = QueueSingle<F>;

impl<X1> S<X1> {
    pub fn into_tuple(self) -> X1 {
        self.front
    }
    pub fn as_tuple(&self) -> &X1 {
        &self.front
    }
    pub fn as_tuple_mut(&mut self) -> &mut X1 {
        &mut self.front
    }
}

impl<X1, X2> Queue<X1, S<X2>> {
    pub fn into_tuple(self) -> (X1, X2) {
        (self.front, self.back.front)
    }
    pub fn as_tuple(&self) -> (&X1, &X2) {
        (&self.front, &self.back.front)
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2) {
        (&mut self.front, &mut self.back.front)
    }
}

impl<X1, X2, X3> Queue<X1, Queue<X2, S<X3>>> {
    pub fn into_tuple(self) -> (X1, X2, X3) {
        (self.front, self.back.front, self.back.back.front)
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3) {
        (&self.front, &self.back.front, &self.back.back.front)
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4> Queue<X1, Queue<X2, Queue<X3, S<X4>>>> {
    pub fn into_tuple(self) -> (X1, X2, X3, X4) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
        )
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5> Queue<X1, Queue<X2, Queue<X3, Queue<X4, S<X5>>>>> {
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
        )
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6> Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, S<X6>>>>>> {
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple_mut(&mut self) -> (&mut X1, &mut X2, &mut X3, &mut X4, &mut X5, &mut X6) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7>
    Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, Queue<X6, S<X7>>>>>>>
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
            self.back.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
    ) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.front,
        )
    }
}

impl<X1, X2, X3, X4, X5, X6, X7, X8>
    Queue<X1, Queue<X2, Queue<X3, Queue<X4, Queue<X5, Queue<X6, Queue<X7, S<X8>>>>>>>>
{
    pub fn into_tuple(self) -> (X1, X2, X3, X4, X5, X6, X7, X8) {
        (
            self.front,
            self.back.front,
            self.back.back.front,
            self.back.back.back.front,
            self.back.back.back.back.front,
            self.back.back.back.back.back.front,
            self.back.back.back.back.back.back.front,
            self.back.back.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple(&self) -> (&X1, &X2, &X3, &X4, &X5, &X6, &X7, &X8) {
        (
            &self.front,
            &self.back.front,
            &self.back.back.front,
            &self.back.back.back.front,
            &self.back.back.back.back.front,
            &self.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.front,
            &self.back.back.back.back.back.back.back.front,
        )
    }
    pub fn as_tuple_mut(
        &mut self,
    ) -> (
        &mut X1,
        &mut X2,
        &mut X3,
        &mut X4,
        &mut X5,
        &mut X6,
        &mut X7,
        &mut X8,
    ) {
        (
            &mut self.front,
            &mut self.back.front,
            &mut self.back.back.front,
            &mut self.back.back.back.front,
            &mut self.back.back.back.back.front,
            &mut self.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.front,
            &mut self.back.back.back.back.back.back.back.front,
        )
    }
}
