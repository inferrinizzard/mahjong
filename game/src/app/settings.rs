use iced::{Element, widget::button};

use crate::{app::root::Message, screens::Screen};

#[derive(Default)]
pub struct Settings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
    pub game: GameSettings,
}

#[derive(Default)]
pub struct VideoSettings {}

#[derive(Default)]
pub struct AudioSettings {}

#[derive(Default)]
pub struct GameSettings {
    pub has_flowers: bool,
    pub has_seasons: bool,
    pub has_joker: bool,
}

#[derive(Debug, Clone)]
pub struct SettingsMessage {}

impl Settings {
    pub fn update(&self, message: SettingsMessage) {
        match message {
            _ => todo!(),
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        button("Back to Main Menu")
            .on_press(Message::ChangeScreen(Screen::Main))
            .into()
    }
}
