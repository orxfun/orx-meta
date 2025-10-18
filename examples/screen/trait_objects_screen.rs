use crate::trait_objects_components::Draw;

pub fn new_screen() -> Vec<Box<dyn Draw>> {
    let path = "examples/screen/trait_objects_components.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}
