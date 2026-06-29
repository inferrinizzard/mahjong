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

pub struct AppRoot<'app> {
    pub render: Render,
    pub settings: Settings,
    pub game: Game<'app>,
    pub server: ServerRoot,
}

impl<'app> AppRoot<'app> {
    pub fn new() -> AppRoot<'app> {
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
            Message::ChangeScreen(screen) => {
                self.render.screen = screen;
                return self.on_render_screen();
            }
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

    fn on_render_screen(&mut self) -> Task<Message> {
        match self.render.screen {
            Screen::Game => {
                self.game.init(&self.settings);
            }
            _ => {}
        }
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screen),
    Counter(CounterMessage),
    Settings(SettingsMessage),
}

pub fn subscription_window_resize(_: &AppRoot) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| {
        Message::Settings(SettingsMessage::VideoSettings(
            VideoSettingsMessage::WindowResize(size),
        ))
    })
}
