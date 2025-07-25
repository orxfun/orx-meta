pub trait DataQueue {
    type Value;

    fn push<X>(self, x: X);

    fn front(&self);

    fn back(&self);
}
