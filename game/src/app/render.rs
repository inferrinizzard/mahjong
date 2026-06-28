use iced::Element;

use crate::{
    app::{
        root::{AppRoot, Message},
        settings::Settings,
    },
    screens::{
        Screen, debug_counter::Counter, debug_tile::DebugTile, main_screen::render_main_screen,
    },
};

pub struct Render {
    pub screen: Screen,
    pub counter: Counter,
    pub debug_tile: DebugTile,
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
    pub fn view(state: &AppRoot) -> Element<'_, Message> {
        match state.render.screen {
            Screen::Main => return render_main_screen(),
            Screen::DebugCounter => return Counter::view(&state.render.counter),
            Screen::DebugTile => return DebugTile::view(&state.render.debug_tile),
            Screen::Settings => return Settings::view(&state.settings),
        }
    }
}
