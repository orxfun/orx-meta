use crate::queue::{Empty, MetaQueue};

pub type MetaQueueOf0 = Empty;

pub type MetaQueueOf1<T1> = <MetaQueueOf0 as MetaQueue>::Push<T1>;

pub type MetaQueueOf2<T1, T2> = <MetaQueueOf1<T1> as MetaQueue>::Push<T2>;

pub type MetaQueueOf3<T1, T2, T3> = <MetaQueueOf2<T1, T2> as MetaQueue>::Push<T3>;

pub type MetaQueueOf4<T1, T2, T3, T4> = <MetaQueueOf3<T1, T2, T3> as MetaQueue>::Push<T4>;

pub type MetaQueueOf5<T1, T2, T3, T4, T5> = <MetaQueueOf4<T1, T2, T3, T4> as MetaQueue>::Push<T5>;

pub type MetaQueueOf6<T1, T2, T3, T4, T5, T6> =
    <MetaQueueOf5<T1, T2, T3, T4, T5> as MetaQueue>::Push<T6>;

pub type MetaQueueOf7<T1, T2, T3, T4, T5, T6, T7> =
    <MetaQueueOf6<T1, T2, T3, T4, T5, T6> as MetaQueue>::Push<T7>;

pub type MetaQueueOf8<T1, T2, T3, T4, T5, T6, T7, T8> =
    <MetaQueueOf7<T1, T2, T3, T4, T5, T6, T7> as MetaQueue>::Push<T8>;
