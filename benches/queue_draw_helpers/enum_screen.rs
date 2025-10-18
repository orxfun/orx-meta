#![allow(dead_code)]

use super::enum_components::Component;

pub fn new_screen_10() -> Vec<Component> {
    let path = "benches/queue_draw_helpers/enum_components_10.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn new_screen_100() -> Vec<Component> {
    let path = "benches/queue_draw_helpers/enum_components_100.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn new_screen_200() -> Vec<Component> {
    let path = "benches/queue_draw_helpers/enum_components_200.json";
    let data = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}
