use iced::Element;

use crate::{
    app::root::Message,
    screens::{debug_counter::Counter, debug_tile::DebugTile, main_screen::render_main_screen},
};

pub struct Render {
    pub screen: Screen,
    pub counter: Counter,
    pub debug_tile: DebugTile,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Main,
    DebugCounter,
    DebugTile,
}

impl Default for Render {
    fn default() -> Self {
        Self {
            screen: Screen::Main,
            counter: Counter::default(),
            debug_tile: DebugTile::default(),
        }
    }
}

impl Render {
    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Main => return render_main_screen(),
            Screen::DebugCounter => return Counter::view(&self.counter),
            Screen::DebugTile => return DebugTile::view(&self.debug_tile),
        }
    }
}
