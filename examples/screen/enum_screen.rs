use crate::enum_components::Component;

pub fn new_screen() -> Vec<Component> {
    let path = "examples/screen/enum_components.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}
