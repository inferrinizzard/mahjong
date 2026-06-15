use crate::{
    app::{game::Game, server::ServerRoot, settings::Settings},
    ui::root::RenderRoot,
};

pub struct AppRoot {
    pub render: RenderRoot,
    pub settings: Settings,
    pub game: Option<Game>,
    pub server: ServerRoot,
}

impl AppRoot {
    pub fn init() -> AppRoot {
        Self {
            render: RenderRoot::default(),
            game: None,
            settings: Settings::default(),
            server: ServerRoot::default(),
        }
    }
}
