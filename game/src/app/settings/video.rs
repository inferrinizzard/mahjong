use iced::{
    Task,
    window::{
        self,
        Mode::{Fullscreen, Windowed},
    },
};

use crate::app::root::Message;

#[derive(Default)]
pub struct VideoSettings {
    pub is_fullscreen: bool,
    // pub window_size:
}

#[derive(Debug, Clone)]
pub enum VideoSettingsMessage {
    ToggleFullscreen,
}

impl VideoSettings {
    pub fn update(&mut self, message: VideoSettingsMessage) -> Task<Message> {
        match message {
            VideoSettingsMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
                let should_be_fullscreen = self.is_fullscreen;
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
}
