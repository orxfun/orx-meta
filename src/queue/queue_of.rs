/// Note that any statically-typed queue of heterogeneous elements can be represented with two [`StQueue`] implementations:
///
/// * [`EmptyQueue`], and
/// * a non-empty [`Queue`].
///
/// This is possible thanks to generic associated types and recursive type definition of the `Queue`.
///
/// On the other hand, it might make it difficult to hand-write the specific queue types.
///
/// Consider for instance the type of a queue containing four elements of types `i32`, `bool`, `char` and `String`.
///
/// The type of this queue would be:
///
/// ```
/// use orx_meta::queue::*;
///
/// type MyQueue = Queue<i32, Queue<bool, Queue<char, Queue<String, EmptyQueue>>>>;
///
/// let instance: MyQueue = Queue::new(42).push(true).push('x').push("foo".to_string());
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
/// ```
///
/// Notice that the nested type definition can get complicated as our queue contains more and more fields.
///
/// `queue_of` macro is a helper macro to make such type aliasing convenient as follows:
///
/// ```
/// use orx_meta::queue::*;
/// use orx_meta::queue_of;
///
/// type MyQueue = queue_of!(i32, bool, char, String);
///
/// let instance: MyQueue = Queue::new(42).push(true).push('x').push("foo".to_string());
/// assert_eq!(instance.as_tuple(), (&42, &true, &'x', &"foo".to_string()));
/// ```
///
/// [`StQueue`]: crate::queue::StQueue
/// [`EmptyQueue`]: crate::queue::EmptyQueue
/// [`Queue`]: crate::queue::Queue
#[macro_export]
macro_rules! queue_of {
    () => {
        EmptyQueue
    };

    ($t1:ty) => {
        Queue<$t1, EmptyQueue>
    };

    ($t1:ty, $t2:ty) => {
        Queue<$t1, Queue<$t2, EmptyQueue>>
    };

    ($t1:ty, $t2:ty, $t3:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, EmptyQueue>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, EmptyQueue>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, EmptyQueue>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, EmptyQueue>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, EmptyQueue>>>>>>>
    };

    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8, EmptyQueue>>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, EmptyQueue>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, EmptyQueue>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, EmptyQueue>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, EmptyQueue>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, EmptyQueue>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, Queue<$t14, EmptyQueue>>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, Queue<$t14, Queue<$t15, EmptyQueue>>>>>>>
        >>>>>>>>
    };

    (
        $t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty, $t7:ty, $t8:ty,
        $t9:ty, $t10:ty, $t11:ty, $t12:ty, $t13:ty, $t14:ty, $t15:ty, $t16:ty
    ) => {
        Queue<$t1, Queue<$t2, Queue<$t3, Queue<$t4, Queue<$t5, Queue<$t6, Queue<$t7, Queue<$t8,
            Queue<$t9, Queue<$t10, Queue<$t11, Queue<$t12, Queue<$t13, Queue<$t14, Queue<$t15, Queue<$t16, EmptyQueue>>>>>>>>
        >>>>>>>>
    };
}
