#![allow(dead_code)]

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Self {
            width,
            height,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("{self:?}");
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self {
            width,
            height,
            options,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}
