use core::any::type_name_of_val;

pub fn format(type_name: impl ToString) -> String {
    type_name
        .to_string()
        .replace("\n", &"")
        .replace(" ", &"")
        .trim()
        .replace("orx_meta::", "")
        .replace("meta_queue::", "")
        .replace("empty::", "")
        .replace("single::", "")
        .replace("pair::", "")
        .replace("alloc::string::", "")
        .replace("alloc::vec::", "")
}

pub fn assert_type<T>(t: &T, type_name: impl ToString) {
    let a = format(type_name_of_val(t));
    let b = format(type_name);
    assert_eq!(a, b);
}
