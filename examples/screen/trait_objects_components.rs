#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "type")]
pub trait Draw {
    fn draw(&self);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

#[typetag::serde]
impl Draw for Button {
    fn draw(&self) {
        println!("{self:?}");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

#[typetag::serde]
impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}
