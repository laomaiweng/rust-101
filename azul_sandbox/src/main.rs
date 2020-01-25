extern crate azul;

use azul::prelude::*;
use azul::widgets::{button::Button, label::Label};

struct CounterApp {
    counter: usize,
}

impl Layout for CounterApp {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Up!")
            .dom()
            .with_callback(On::MouseUp, update_counter);
        Dom::div().with_child(label).with_child(button)
    }
}

fn update_counter(info: CallbackInfo<CounterApp>) -> UpdateScreen {
    info.state.data.counter += 1;
    Redraw
}

fn main() {
    let mut app = App::new(CounterApp { counter: 0 }, AppConfig::default()).unwrap();
    let window = app
        .create_window(WindowCreateOptions::default(), css::native())
        .unwrap();
    app.run(window).unwrap();
}
