use iced::{
    Element,
    widget::{button, column, text},
};

use crate::app::{render::Screen, root::Message};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Element<'static, Message> {
        // We use a column: a simple vertical layout
        column![
            button("Back to Main").on_press(Message::ChangeScreen(Screen::Main)),
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Counter(CounterMessage::Increment)),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Counter(CounterMessage::Decrement)),
        ]
        .into()
    }

    pub fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => {
                self.value += 1;
            }
            CounterMessage::Decrement => {
                self.value -= 1;
            }
        }
    }
}
