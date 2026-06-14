pub mod app;
pub mod screens;
pub mod util;

use iced::window::{Settings, icon};

use crate::{app::root::AppRoot, util::get_path::get_path};

pub fn main() -> iced::Result {
    iced::application(AppRoot::default, AppRoot::update, AppRoot::view)
        .title("Mahjong")
        .window(Settings {
            icon: Some(icon::from_file(get_path("assets/icon.ico")).unwrap()),
            ..Settings::default()
        })
        .run()
}
