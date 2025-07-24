use crate::queue::{Empty, MetaQueue};

/// Creates the [`MetaQueue`] with no types; i.e., returns [`Empty`].
pub type MetaQueueFrom0 = Empty;

/// Creates the [`MetaQueue`] with a single type; i.e., returns [`One`] of `T1`.
///
/// [`One`]: crate::queue::One
pub type MetaQueueFrom1<T1> = <MetaQueueFrom0 as MetaQueue>::Push<T1>;

/// Creates the [`MetaQueue`] with two types; i.e., returns [`Multi`] of `T1` and `T2`.
///
/// [`Multi`]: crate::queue::Multi
pub type MetaQueueFrom2<T1, T2> = <MetaQueueFrom1<T1> as MetaQueue>::Push<T2>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom3<T1, T2, T3> = <MetaQueueFrom2<T1, T2> as MetaQueue>::Push<T3>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom4<T1, T2, T3, T4> = <MetaQueueFrom3<T1, T2, T3> as MetaQueue>::Push<T4>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom5<T1, T2, T3, T4, T5> =
    <MetaQueueFrom4<T1, T2, T3, T4> as MetaQueue>::Push<T5>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom6<T1, T2, T3, T4, T5, T6> =
    <MetaQueueFrom5<T1, T2, T3, T4, T5> as MetaQueue>::Push<T6>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom7<T1, T2, T3, T4, T5, T6, T7> =
    <MetaQueueFrom6<T1, T2, T3, T4, T5, T6> as MetaQueue>::Push<T7>;

/// Creates the [`MetaQueue`] of the given type arguments.
///
/// # Examples
///
/// ```
/// use orx_meta::queue::*;
///
/// fn format(type_name: impl ToString) -> String {
///     type_name
///         .to_string()
///         .replace("\n", &"")
///         .replace(" ", &"")
///         .trim()
///         .replace("orx_meta::", "")
///         .replace("queue::", "")
///         .replace("empty::", "")
///         .replace("one::", "")
///         .replace("multi::", "")
///         .replace("alloc::string::", "")
///         .replace("alloc::vec::", "")
/// }
///
/// fn assert_type<T>(t: &T, type_name: impl ToString) {
///     let a = format(core::any::type_name_of_val(t));
///     let b = format(type_name);
///     assert_eq!(a, b);
/// }
///
/// let x = MetaQueueFrom6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueFrom8<T1, T2, T3, T4, T5, T6, T7, T8> =
    <MetaQueueFrom7<T1, T2, T3, T4, T5, T6, T7> as MetaQueue>::Push<T8>;
