pub mod consts;
pub mod render_tile;

use iced::Element;

use crate::{
    app::{
        game::Game,
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
    pub fn view(state: &AppRoot) -> Element<'static, Message> {
        match state.render.screen {
            Screen::Main => render_main_screen(),
            Screen::Game => Game::view(&state.game, &state.settings),
            Screen::DebugCounter => Counter::view(&state.render.counter),
            Screen::DebugTile => DebugTile::view(&state.render.debug_tile),
            Screen::Settings => Settings::view(&state.settings),
        }
    }
}
