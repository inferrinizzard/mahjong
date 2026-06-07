use iced::Element;

use crate::screens::{
    debug_counter::{Counter, CounterMessage},
    main_screen::render_main_screen,
};

pub struct AppRoot {
    pub screen: Screen,
    pub counter: Counter,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    Counter(CounterMessage),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Main,
    DebugCounter,
}

impl Default for AppRoot {
    fn default() -> Self {
        Self {
            screen: Screen::Main,
            counter: Counter::default(),
        }
    }
}

impl AppRoot {
    pub fn update(&mut self, message: Message) {
        println!("{:?}", message);

        match message {
            Message::ChangeScreen(screen) => self.screen = screen,
            Message::Counter(counter_message) => {
                Counter::update(&mut self.counter, counter_message)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => return render_main_screen().into(),
            Screen::DebugCounter => return Counter::view(&self.counter).into(),
        }
    }
}
