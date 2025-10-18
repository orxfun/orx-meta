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
        if self.width == self.height {
            println!("{self:?}");
        } else if self.label.starts_with('x') {
            println!("{self:?}");
        }
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
        if self.width == self.height {
            println!("{self:?}");
        }

        if let Some(x) = self.options.get(1) {
            if x.starts_with('x') {
                println!("{self:?}");
            }
        }

        if let Some(x) = self.options.get(0) {
            if x.starts_with('y') || x.ends_with('x') {
                println!("{self:?}");
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

#[typetag::serde]
impl Draw for Label {
    fn draw(&self) {
        if self.width == self.height {
            println!("{self:?}");
        } else if self.label.starts_with('x') {
            println!("{self:?}");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckBox {
    pub width: u32,
    pub height: u32,
    pub is_checked: bool,
}

#[typetag::serde]
impl Draw for CheckBox {
    fn draw(&self) {
        if self.width == self.height {
            println!("{self:?}");
        } else if self.is_checked && self.width == 13 {
            println!("{self:?}");
        }
    }
}
