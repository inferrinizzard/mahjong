use iced::{Element, Subscription, Task, window};

use crate::{
    app::{
        game::Game,
        render::Render,
        server::ServerRoot,
        settings::{Settings, SettingsMessage, video::VideoSettingsMessage},
    },
    screens::{
        Screen,
        debug_counter::{Counter, CounterMessage},
    },
};

pub struct AppRoot {
    pub render: Render,
    pub settings: Settings,
    pub game: Game,
    pub server: ServerRoot,
}

impl AppRoot {
    pub fn new() -> AppRoot {
        Self {
            render: Render::default(),
            game: Game::default(),
            settings: Settings::default(),
            server: ServerRoot::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message);

        match message {
            Message::ChangeScreen(screen) => self.render.screen = screen,
            Message::Counter(counter_message) => {
                Counter::update(&mut self.render.counter, counter_message)
            }
            Message::Settings(settings_message) => {
                return Settings::update(&mut self.settings, settings_message);
            }
        }

        Task::none()
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

pub fn subscription_window_resize() -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| {
        Message::Settings(SettingsMessage::VideoSettings(
            VideoSettingsMessage::WindowResize(size),
        ))
    })
}
