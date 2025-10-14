use crate::product::push::Push;

pub trait Pop: Push {
    type Front;

    type PopFront: Push;

    type Back;

    type PopBack: Push;

    // into

    fn pop_back(self) -> (Self::Back, Self::PopBack);

    fn pop_front(self) -> (Self::Front, Self::PopFront);
}
