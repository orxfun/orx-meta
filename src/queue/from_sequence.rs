use crate::queue::{Empty, MetaQueue};

/// Creates the [`MetaQueue`] with no types; i.e., returns [`Empty`].
pub type MetaQueueOf0 = Empty;

/// Creates the [`MetaQueue`] with a single type; i.e., returns [`One`] of `T1`.
///
/// [`One`]: crate::queue::One
pub type MetaQueueOf1<T1> = <MetaQueueOf0 as MetaQueue>::Push<T1>;

/// Creates the [`MetaQueue`] with two types; i.e., returns [`Multi`] of `T1` and `T2`.
///
/// [`Multi`]: crate::queue::Multi
pub type MetaQueueOf2<T1, T2> = <MetaQueueOf1<T1> as MetaQueue>::Push<T2>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf3<T1, T2, T3> = <MetaQueueOf2<T1, T2> as MetaQueue>::Push<T3>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf4<T1, T2, T3, T4> = <MetaQueueOf3<T1, T2, T3> as MetaQueue>::Push<T4>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf5<T1, T2, T3, T4, T5> = <MetaQueueOf4<T1, T2, T3, T4> as MetaQueue>::Push<T5>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf6<T1, T2, T3, T4, T5, T6> =
    <MetaQueueOf5<T1, T2, T3, T4, T5> as MetaQueue>::Push<T6>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf7<T1, T2, T3, T4, T5, T6, T7> =
    <MetaQueueOf6<T1, T2, T3, T4, T5, T6> as MetaQueue>::Push<T7>;

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
/// let x = MetaQueueOf6::<char, u32, String, bool, u8, f64>::default();
/// assert_type(
///     &x,
///     "Multi<char,Multi<u32,Multi<String,Multi<bool,Multi<u8,One<f64>>>>>>",
/// );
/// ```
pub type MetaQueueOf8<T1, T2, T3, T4, T5, T6, T7, T8> =
    <MetaQueueOf7<T1, T2, T3, T4, T5, T6, T7> as MetaQueue>::Push<T8>;
