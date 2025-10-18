use super::trait_objects_components::Draw;

pub fn new_screen_10() -> Vec<Box<dyn Draw>> {
    let path = "benches/queue_draw_helpers/trait_objects_components_10.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn new_screen_100() -> Vec<Box<dyn Draw>> {
    let path = "benches/queue_draw_helpers/trait_objects_components_100.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn new_screen_200() -> Vec<Box<dyn Draw>> {
    let path = "benches/queue_draw_helpers/trait_objects_components_200.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}
