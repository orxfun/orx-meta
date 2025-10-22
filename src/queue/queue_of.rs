/// Recall that there exist two statically-typed queue (`StQueue`) implementations:
///
/// * `QueueSingle` which includes exactly one element, and
/// * `Queue` containing multiple (>=2) elements.
///
/// Queues of all lengths can be represented by these two types:
/// * `QueueSingle<T1>` is a queue with one element,
/// * `Queue<T1, QueueSingle<T2>>` with two elements,
/// * `Queue<T1, Queue<T2, QueueSingle<T3>>>` with three elements,
/// * `Queue<T1, Queue<T2, Queue<T3, QueueSingle<T4>>>>` with four elements,
/// * and so on, so forth.
///
/// This is possible thanks to generic associated types and recursive type definition of the `Queue`.
///
/// On the other hand, it might make it difficult to hand-write queue types.
///
/// `queue_of` macro is a helper macro to make such type aliasing convenient whenever needed.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
/// use orx_meta::queue_of;
///
/// // written with recursive type definition
/// type Q1 = Queue<i32, Queue<bool, Queue<char, QueueSingle<String>>>>;
///
/// let instance: Q1 = Queue::new(42).push(true).push('x').push("foo".to_string());
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
///
/// // alternatively, using with queue_of macro as a flat list
/// type Q2 = queue_of!(i32, bool, char, String);
///
/// // notice that Q1 and Q2 are aliases for the same type
/// let instance2: Q2 = instance;
/// ```
#[macro_export]
macro_rules! queue_of {
    ($t1:ty) => {
        QueueSingle<$t1>
    };

    ($t1:ty, $t2:ty) => {
        Queue<$t1, QueueSingle<$t2>>
    };

    ($t1:ty, $t2:ty, $t3:ty) => {
        Queue<$t1, Queue<$t2, QueueSingle<$t3>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, QueueSingle<$t4>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, QueueSingle<$t5>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, QueueSingle<$t6>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, QueueSingle<$t7>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, QueueSingle<$t8>>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            QueueSingle<$t9>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, QueueSingle<$t10>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, QueueSingle<$t11>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, QueueSingle<$t12>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, QueueSingle<$t13>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, QueueSingle<$t14>>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, Queue<$t14, QueueSingle<$t15>>>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, Queue<$t14, Queue<$t15, QueueSingle<$t16>>>>>>>>
        >>>>>>>>
    };
}
