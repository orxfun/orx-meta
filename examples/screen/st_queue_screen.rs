use crate::st_queue_components::{Button, Draw, SelectBox};

orx_meta::define_nonempty_queue!(
    elements => [ Draw ];
    queue => [ StScreen; Single, Screen ];
);

impl<F: Draw> Draw for Single<F> {
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

pub fn new_screen() -> impl Draw {
    Single::new(Button {
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
        width: 6,
        height: 6,
        label: "login".to_string(),
    })
}
