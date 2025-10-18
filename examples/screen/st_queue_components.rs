#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{self:?}");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}
