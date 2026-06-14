use iced::Element;

use crate::screens::{
    debug_counter::{Counter, CounterMessage},
    debug_tile::DebugTile,
    main_screen::render_main_screen,
};

pub struct RenderRoot {
    pub screen: Screen,
    pub counter: Counter,
    pub debug_tile: DebugTile,
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
    DebugTile,
}

impl Default for RenderRoot {
    fn default() -> Self {
        Self {
            screen: Screen::Main,
            counter: Counter::default(),
            debug_tile: DebugTile::default(),
        }
    }
}

impl RenderRoot {
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
            Screen::Main => return render_main_screen(),
            Screen::DebugCounter => return Counter::view(&self.counter),
            Screen::DebugTile => return DebugTile::view(&self.debug_tile),
        }
    }
}
