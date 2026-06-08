use iced::{
    Element,
    widget::{column, row, svg},
};

use crate::app::root::Message;

pub struct DebugTile {}

impl Default for DebugTile {
    fn default() -> Self {
        DebugTile {}
    }
}

impl DebugTile {
    pub fn view(&self) -> Element<'static, Message> {
        column![
            row![
                svg("assets/tiles/flat/1m.svg"),
                svg("assets/tiles/flat/2m.svg"),
                svg("assets/tiles/flat/3m.svg"),
                svg("assets/tiles/flat/4m.svg"),
                svg("assets/tiles/flat/5m.svg"),
                svg("assets/tiles/flat/6m.svg"),
                svg("assets/tiles/flat/7m.svg"),
                svg("assets/tiles/flat/8m.svg"),
                svg("assets/tiles/flat/9m.svg"),
            ],
            row![
                svg("assets/tiles/flat/1p.svg"),
                svg("assets/tiles/flat/2p.svg"),
                svg("assets/tiles/flat/3p.svg"),
                svg("assets/tiles/flat/4p.svg"),
                svg("assets/tiles/flat/5p.svg"),
                svg("assets/tiles/flat/6p.svg"),
                svg("assets/tiles/flat/7p.svg"),
                svg("assets/tiles/flat/8p.svg"),
                svg("assets/tiles/flat/9p.svg"),
            ],
            row![
                svg("assets/tiles/flat/1s.svg"),
                svg("assets/tiles/flat/2s.svg"),
                svg("assets/tiles/flat/3s.svg"),
                svg("assets/tiles/flat/4s.svg"),
                svg("assets/tiles/flat/5s.svg"),
                svg("assets/tiles/flat/6s.svg"),
                svg("assets/tiles/flat/7s.svg"),
                svg("assets/tiles/flat/8s.svg"),
                svg("assets/tiles/flat/9s.svg"),
            ],
            row![
                svg("assets/tiles/flat/east.svg"),
                svg("assets/tiles/flat/south.svg"),
                svg("assets/tiles/flat/west.svg"),
                svg("assets/tiles/flat/north.svg"),
                svg("assets/tiles/flat/white.svg"),
                svg("assets/tiles/flat/green.svg"),
                svg("assets/tiles/flat/red.svg"),
                svg("assets/tiles/flat/joker.svg"),
                svg("assets/tiles/flat/r5p.svg"),
                svg("assets/tiles/flat/r5s.svg"),
            ],
        ]
        .into()
    }
}
