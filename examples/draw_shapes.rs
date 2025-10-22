#![allow(dead_code)]

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    fn new(width: u32, height: u32, label: String) -> Self {
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
struct SelectBox {
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

        pub fn draw(&self) {
            for component in &self.components {
                component.draw();
            }
        }
    }

    let mut screen = Screen::new();
    screen.push(Box::new(Button::new(3, 4, "home".to_string())));
    screen.push(Box::new(Button::new(5, 4, "about".to_string())));
    screen.push(Box::new(SelectBox::new(5, 4, vec!["one".to_string()])));
    screen.push(Box::new(Button::new(6, 6, "login".to_string())));
    screen.draw();

    // prints out:
    //
    // Button { width: 3, height: 4, label: "home" }
    // Button { width: 5, height: 4, label: "about" }
    // SelectBox { width: 5, height: 4, options: ["one] }
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
    screen.push(Component::Button(Button::new(3, 4, "home".to_string())));
    screen.push(Component::Button(Button::new(5, 4, "about".to_string())));
    screen.push(Component::SelectBox(SelectBox::new(
        5,
        4,
        vec!["one".to_string()],
    )));
    screen.push(Component::Button(Button::new(6, 6, "login".to_string())));
    screen.draw();
}

fn solution_with_composition() {
    println!("\n# solution_with_composition");

    orx_meta::define_queue!(
        elements => [ Draw ];
        queue => [ StScreen; ScreenSingle, Screen ];
    );

    impl<F: Draw> Draw for ScreenSingle<F> {
        // identity: do nothing
        fn draw(&self) {
            self.f.draw();
        }
    }

    impl<F: Draw, B: StScreen> Draw for Screen<F, B> {
        // composition: draw them both
        fn draw(&self) {
            self.f.draw();
            self.b.draw();
        }
    }

    let screen = Screen::new(Button::new(3, 4, "home".to_string()))
        .push(Button::new(5, 4, "about".to_string()))
        .push(SelectBox::new(5, 4, vec!["one".to_string()]))
        .push(Button::new(6, 6, "login".to_string()));
    screen.draw();
}

fn solution_with_composition_hand_written() {
    println!("\n# solution_with_composition_hand_written");
    struct Screen {
        btn1: Button,
        btn2: Button,
        sbox: SelectBox,
        btn3: Button,
    }

    impl Screen {
        fn draw(&self) {
            self.btn1.draw();
            self.btn2.draw();
            self.sbox.draw();
            self.btn3.draw();
        }
    }

    let screen = Screen {
        btn1: Button::new(3, 4, "home".to_string()),
        btn2: Button::new(5, 4, "about".to_string()),
        sbox: SelectBox::new(5, 4, vec!["one".to_string()]),
        btn3: Button::new(6, 6, "login".to_string()),
    };
    screen.draw();
}

fn solution_with_composition_expansion() {
    println!("\n# solution_with_composition_expansion");

    trait StScreen: Draw {
        type PushBack<T>: StScreen
        where
            T: Draw;

        type Front;

        type Back: StScreen;

        const LEN: usize;

        fn len(&self) -> usize {
            Self::LEN
        }

        fn push<T>(self, element: T) -> Self::PushBack<T>
        where
            T: Draw;
    }

    struct EmptyScreen;

    impl StScreen for EmptyScreen {
        type PushBack<T>
            = Screen<T, EmptyScreen>
        where
            T: Draw;

        type Front = Self;

        type Back = Self;

        const LEN: usize = 0;

        fn push<T>(self, element: T) -> Self::PushBack<T>
        where
            T: Draw,
        {
            Screen {
                f: element,
                b: EmptyScreen,
            }
        }
    }

    pub struct Screen<F, B>
    where
        F: Draw,
        B: StScreen,
    {
        f: F,
        b: B,
    }

    impl<F, B> StScreen for Screen<F, B>
    where
        F: Draw,
        B: StScreen,
    {
        type PushBack<T>
            = Screen<F, B::PushBack<T>>
        where
            T: Draw;

        type Front = F;

        type Back = B;

        const LEN: usize = 1 + B::LEN;

        fn push<T>(self, element: T) -> Self::PushBack<T>
        where
            T: Draw,
        {
            Screen {
                f: self.f,
                b: self.b.push(element),
            }
        }
    }

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

    let screen = EmptyScreen
        .push(Button::new(3, 4, "home".to_string()))
        .push(Button::new(5, 4, "about".to_string()))
        .push(SelectBox::new(5, 4, vec!["one".to_string()]))
        .push(Button::new(6, 6, "login".to_string()));
    screen.draw();
}

fn main() {
    solution_with_trait_objects();
    solution_with_enums();
    solution_with_composition();
    solution_with_composition_hand_written();
    solution_with_composition_expansion();
}
