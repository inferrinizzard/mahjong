use iced::Element;

use crate::screens::main_screen::render_main_screen;

pub struct AppRoot {
    pub screen: Screen,
}

#[derive(Clone)]
pub enum Message {}

pub enum Screen {
    Main,
    // DebugCounter,
}

impl Default for AppRoot {
    fn default() -> Self {
        Self {
            screen: Screen::Main,
        }
    }
}

impl AppRoot {
    pub fn update(&mut self, event: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        match (self.screen) {
            Screen::Main => return render_main_screen(),
            // Screen::DebugCounter => {}
        }
    }
}
