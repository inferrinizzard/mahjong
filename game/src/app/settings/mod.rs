pub mod video;
pub use video::{VideoSettings, VideoSettingsMessage};

use derivative::Derivative;
use iced::{
    Task,
    widget::{button, column},
};

use crate::{
    app::{Component, Message},
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

#[derive(Derivative)]
#[derivative(Default)]
pub struct GameSettings {
    #[derivative(Default(value = "13"))]
    pub hand_size: usize,

    #[derivative(Default(value = "true"))]
    pub has_flowers: bool,
    #[derivative(Default(value = "true"))]
    pub has_seasons: bool,
    #[derivative(Default(value = "false"))]
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

    pub fn view(&self) -> Component {
        column!(
            button("Back to Main Menu").on_press(Message::ChangeScreen(Screen::Main)),
            button("Toggle Fullscreen").on_press(Message::Settings(
                SettingsMessage::VideoSettings(VideoSettingsMessage::ToggleFullscreen)
            )),
        )
        .into()
    }
}
