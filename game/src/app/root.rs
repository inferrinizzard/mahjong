use iced::Element;

use crate::{
    app::{
        game::Game,
        render::Render,
        server::ServerRoot,
        settings::{Settings, SettingsMessage},
    },
    screens::{
        Screen,
        debug_counter::{Counter, CounterMessage},
    },
};

pub struct AppRoot {
    pub render: Render,
    pub settings: Settings,
    pub game: Option<Game>,
    pub server: ServerRoot,
}

impl AppRoot {
    pub fn new() -> AppRoot {
        Self {
            render: Render::default(),
            game: None,
            settings: Settings::default(),
            server: ServerRoot::default(),
        }
    }

    pub fn update(&mut self, message: Message) {
        println!("{:?}", message);

        match message {
            Message::ChangeScreen(screen) => self.render.screen = screen,
            Message::Counter(counter_message) => {
                Counter::update(&mut self.render.counter, counter_message)
            }
            Message::Settings(settings_message) => {
                Settings::update(&mut self.settings, settings_message)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Render::view(&self)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    Counter(CounterMessage),
    Settings(SettingsMessage),
}
