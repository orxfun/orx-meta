#[doc(hidden)]
#[macro_export]
macro_rules! test_nonempty_queue_of {
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
