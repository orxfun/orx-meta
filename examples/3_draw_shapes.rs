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
        println!("{self:?}");
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{self:?}");
    }
}

fn dynamic_dispatch() {
    #[derive(Default)]
    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        fn draw(&self) {
            for c in &self.components {
                c.draw();
            }
        }
    }

    let mut screen = Screen::default();

    screen.components.push(Box::new(Button {
        width: 3,
        height: 4,
        label: String::from("login"),
    }));

    screen.components.push(Box::new(Button {
        width: 4,
        height: 5,
        label: String::from("logout"),
    }));

    screen.components.push(Box::new(SelectBox {
        width: 10,
        height: 6,
        options: vec![String::from("This"), String::from("that")],
    }));

    screen.draw();

    // prints out:
    //
    // Button { width: 3, height: 4, label: "login" }
    // Button { width: 4, height: 5, label: "logout" }
    // SelectBox { width: 10, height: 6, options: ["This", "that"] }
}

fn static_dispatch() {
    //
}

fn main() {
    dynamic_dispatch();
}
