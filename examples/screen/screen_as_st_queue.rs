use crate::components::{Button, Draw, SelectBox};

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

pub fn new_screen() -> impl Draw {
    EmptyScreen::new()
        .push(Button::new(3, 4, "home".to_string()))
        .push(Button::new(5, 4, "about".to_string()))
        .push(SelectBox::new(5, 4, vec!["one".to_string()]))
        .push(Button::new(6, 6, "login".to_string()))
}
