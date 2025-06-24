use crate::MetaQueue;
use core::any::type_name_of_val;

pub fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_meta_queue::", "")
}

pub fn assert_type<T>(t: &T, type_name: impl ToString) {
    let a = format(type_name_of_val(t));
    let b = format(type_name);
    assert_eq!(a, b);
}

pub fn push<Q, T>(_: Q) -> <Q as MetaQueue>::Push<T>
where
    Q: MetaQueue,
{
    Default::default()
}

pub fn pop<Q>(_: Q) -> (<Q as MetaQueue>::Front, <Q as MetaQueue>::Back)
where
    Q: MetaQueue,
    <Q as MetaQueue>::Front: Default,
{
    Default::default()
}
