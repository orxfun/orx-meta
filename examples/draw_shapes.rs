pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
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

fn solution_with_trait_objects() {
    println!("\n# solution_with_trait_objects");
    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn new() -> Self {
            Self { components: vec![] }
        }

        pub fn push(&mut self, component: Box<dyn Draw>) {
            self.components.push(component);
        }

        pub fn run(&self) {
            for component in &self.components {
                component.draw();
            }
        }
    }

    let mut screen = Screen::new();
    screen.push(Box::new(Button {
        width: 3,
        height: 4,
        label: "home".to_string(),
    }));
    screen.push(Box::new(Button {
        width: 5,
        height: 4,
        label: "about".to_string(),
    }));
    screen.push(Box::new(SelectBox {
        width: 5,
        height: 4,
        options: vec!["this".to_string(), "that".to_string()],
    }));
    screen.push(Box::new(Button {
        width: 6,
        height: 6,
        label: "login".to_string(),
    }));
    screen.run();

    // prints out:
    //
    // Button { width: 3, height: 4, label: "home" }
    // Button { width: 5, height: 4, label: "about" }
    // SelectBox { width: 5, height: 4, options: ["this", "that"] }
    // Button { width: 6, height: 6, label: "login" }
}

fn solution_with_enums() {
    println!("\n# solution_with_enums");

    enum Component {
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

    struct Screen {
        components: Vec<Component>,
    }

    impl Screen {
        pub fn new() -> Self {
            Self { components: vec![] }
        }

        pub fn push(&mut self, component: Component) {
            self.components.push(component);
        }

        pub fn draw(&self) {
            for component in &self.components {
                component.draw();
            }
        }
    }

    let mut screen = Screen::new();
    screen.push(Component::Button(Button {
        width: 3,
        height: 4,
        label: "home".to_string(),
    }));
    screen.push(Component::Button(Button {
        width: 5,
        height: 4,
        label: "about".to_string(),
    }));
    screen.push(Component::SelectBox(SelectBox {
        width: 5,
        height: 4,
        options: vec!["this".to_string(), "that".to_string()],
    }));
    screen.push(Component::Button(Button {
        width: 6,
        height: 6,
        label: "login".to_string(),
    }));
    screen.draw();
}

fn solution_with_composition() {
    println!("\n# solution_with_composition");

    orx_meta::define_queue!(
        elements => [ Draw ];
        queue => [ StScreen; EmptyScreen, Screen ];
    );

    impl Draw for EmptyScreen {
        // identity: do nothing
        fn draw(&self) {}
    }

    impl<F: Draw, B: StScreen> Draw for Screen<F, B> {
        // composition: draw them both
        fn draw(&self) {
            self.f.draw();
            self.b.draw();
        }
    }

    let screen = EmptyScreen::new()
        .push(Button {
            width: 3,
            height: 4,
            label: "home".to_string(),
        })
        .push(Button {
            width: 5,
            height: 4,
            label: "about".to_string(),
        })
        .push(SelectBox {
            width: 5,
            height: 4,
            options: vec!["this".to_string(), "that".to_string()],
        })
        .push(Button {
            width: 6,
            height: 6,
            label: "login".to_string(),
        });
    screen.draw();
}

fn main() {
    solution_with_trait_objects();
    solution_with_enums();
    solution_with_composition();
}
