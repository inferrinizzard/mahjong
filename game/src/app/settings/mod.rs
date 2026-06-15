pub mod video;

use iced::{
    Element, Task,
    widget::{button, column},
};

use crate::{
    app::{
        root::Message,
        settings::video::{VideoSettings, VideoSettingsMessage},
    },
    screens::Screen,
};

#[derive(Default)]
pub struct Settings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
    pub game: GameSettings,
}

#[derive(Default)]
pub struct AudioSettings {}

#[derive(Default)]
pub struct GameSettings {
    pub has_flowers: bool,
    pub has_seasons: bool,
    pub has_joker: bool,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    VideoSettings(VideoSettingsMessage),
}

impl Settings {
    pub fn update(&mut self, message: SettingsMessage) -> Task<Message> {
        match message {
            SettingsMessage::VideoSettings(video_settings_message) => {
                VideoSettings::update(&mut self.video, video_settings_message)
            }
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        column!(
            button("Back to Main Menu").on_press(Message::ChangeScreen(Screen::Main)),
            button("Toggle Fullscreen").on_press(Message::Settings(
                SettingsMessage::VideoSettings(VideoSettingsMessage::ToggleFullscreen)
            )),
        )
        .into()
    }
}
