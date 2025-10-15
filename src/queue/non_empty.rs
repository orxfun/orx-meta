use crate::queue::{EmptyQueue, st_queue::StQueue};

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
    type PushBack<T> = Queue<F, B::PushBack<T>>;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    fn push<T>(self, element: T) -> Self::PushBack<T> {
        Queue::from((self.front, self.back.push(element)))
    }
}

impl<F> Queue<F, EmptyQueue> {
    pub fn new(front: F) -> Self {
        Self {
            front,
            back: EmptyQueue,
        }
    }
}

impl<F, B> From<(F, B)> for Queue<F, B>
where
    B: StQueue,
{
    fn from((front, back): (F, B)) -> Self {
        Self { front, back }
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    // ref

    pub fn front(&self) -> &F {
        &self.front
    }

    pub fn back(&self) -> &B {
        &self.back
    }

    // mut

    pub fn front_mut(&mut self) -> &mut F {
        &mut self.front
    }

    pub fn back_mut(&mut self) -> &mut B {
        &mut self.back
    }

    // into

    pub fn into_front(self) -> F {
        self.front
    }

    pub fn into_back(self) -> B {
        self.back
    }

    pub fn pop(self) -> (F, B) {
        (self.front, self.back)
    }

    pub fn pop_back(self) {
        todo!()
    }
}

// tuple

type S<F> = Queue<F, EmptyQueue>;

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
