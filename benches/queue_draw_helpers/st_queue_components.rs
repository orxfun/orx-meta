pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        if self.width == self.height {
            println!("{self:?}");
        } else if self.label.starts_with('x') {
            println!("{self:?}");
        }
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

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

#[derive(Debug)]
pub struct Label {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Label {
    fn draw(&self) {
        if self.width == self.height {
            println!("{self:?}");
        } else if self.label.starts_with('x') {
            println!("{self:?}");
        }
    }
}

#[derive(Debug)]
pub struct CheckBox {
    pub width: u32,
    pub height: u32,
    pub is_checked: bool,
}

impl Draw for CheckBox {
    fn draw(&self) {
        if self.width == self.height {
            println!("{self:?}");
        } else if self.is_checked && self.width == 13 {
            println!("{self:?}");
        }
    }
}
