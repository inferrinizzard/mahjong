use iced::{
    Element, Task,
    widget::{button, column},
    window::{
        self,
        Mode::{Fullscreen, Windowed},
    },
};

use crate::{app::root::Message, screens::Screen};

#[derive(Default)]
pub struct Settings {
    pub video: VideoSettings,
    pub audio: AudioSettings,
    pub game: GameSettings,
}

#[derive(Default)]
pub struct VideoSettings {
    pub is_fullscreen: bool,
    // pub window_size:
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
    ToggleFullscreen,
}

impl Settings {
    pub fn update(&mut self, message: SettingsMessage) -> Task<Message> {
        match message {
            SettingsMessage::ToggleFullscreen => {
                self.video.is_fullscreen = !self.video.is_fullscreen;
                let should_be_fullscreen = self.video.is_fullscreen;
                window::latest().and_then(move |window_id| {
                    window::set_mode(
                        window_id,
                        if should_be_fullscreen {
                            Fullscreen
                        } else {
                            Windowed
                        },
                    )
                })
            }
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        column!(
            button("Back to Main Menu").on_press(Message::ChangeScreen(Screen::Main)),
            button("Toggle Fullscreen")
                .on_press(Message::Settings(SettingsMessage::ToggleFullscreen)),
        )
        .into()
    }
}
