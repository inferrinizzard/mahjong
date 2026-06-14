pub mod app;
pub mod screens;
pub mod ui;
pub mod util;

use iced::window::{Settings, icon};

use crate::{ui::root::RenderRoot, util::get_path::get_path};

pub fn main() -> iced::Result {
    iced::application(RenderRoot::default, RenderRoot::update, RenderRoot::view)
        .title("Mahjong")
        .window(Settings {
            icon: Some(icon::from_file(get_path("assets/icon.ico")).unwrap()),
            ..Settings::default()
        })
        .run()
}
