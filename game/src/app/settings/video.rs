use iced::{
    Size, Task,
    window::{
        self,
        Mode::{Fullscreen, Windowed},
    },
};

use crate::app::root::Message;

#[derive(Default)]
pub struct VideoSettings {
    pub is_fullscreen: bool,
    pub window_size: Size,
}

#[derive(Debug, Clone)]
pub enum VideoSettingsMessage {
    ToggleFullscreen,
    WindowResize(Size),
}

impl VideoSettings {
    pub fn update(&mut self, message: VideoSettingsMessage) -> Task<Message> {
        match message {
            VideoSettingsMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
                let should_be_fullscreen = self.is_fullscreen;
                return window::latest().and_then(move |window_id| {
                    window::set_mode(
                        window_id,
                        if should_be_fullscreen {
                            Fullscreen
                        } else {
                            Windowed
                        },
                    )
                });
            }
            VideoSettingsMessage::WindowResize(size) => self.window_size = size,
        }

        Task::none()
    }
}
