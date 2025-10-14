use crate::product::pop::Pop;

pub trait Push {
    type PushBack<X>: Pop;

    type PushFront<X>: Pop;

    const LEN: usize;

    fn len(&self) -> usize {
        Self::LEN
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push_back<X>(self, element: X) -> Self::PushBack<X>;

    fn push_front<X>(self, element: X) -> Self::PushFront<X>;
}
