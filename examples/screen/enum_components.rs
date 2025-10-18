#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

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

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}

// enum

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Component {
    Button(Button),
    SelectBox(SelectBox),
}

impl Draw for Component {
    fn draw(&self) {
        match self {
            Self::Button(x) => x.draw(),
            Self::SelectBox(x) => x.draw(),
        }
    }
}
