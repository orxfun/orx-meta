use super::st_queue_components::*;

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

pub fn new_screen_10() -> (usize, impl Draw) {
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        });
    (screen.len(), screen)
}

pub fn new_screen_100() -> (usize, impl Draw) {
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        });
    (screen.len(), screen)
}

pub fn new_screen_200() -> (usize, impl Draw) {
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        })
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
            options: vec!["one".to_string()],
        })
        .push(Button {
            width: 7,
            height: 6,
            label: "login".to_string(),
        })
        .push(Label {
            width: 7,
            height: 6,
            label: "hello".to_string(),
        })
        .push(Label {
            width: 11,
            height: 2,
            label: "world".to_string(),
        })
        .push(Button {
            width: 1,
            height: 7,
            label: "contact".to_string(),
        })
        .push(CheckBox {
            width: 5,
            height: 9,
            is_checked: true,
        })
        .push(CheckBox {
            width: 2,
            height: 3,
            is_checked: false,
        })
        .push(Button {
            width: 7,
            height: 1,
            label: "contact-2".to_string(),
        });
    (screen.len(), screen)
}
