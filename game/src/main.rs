pub mod app;
pub mod screens;
pub mod util;

use iced::window::{Settings, icon};

use crate::{
    app::root::{AppRoot, subscription_window_resize},
    util::get_path::get_path,
};

pub fn main() -> iced::Result {
    iced::application(AppRoot::new, AppRoot::update, AppRoot::view)
        .title("Mahjong")
        .window(Settings {
            icon: Some(icon::from_file(get_path("assets/icon.ico")).unwrap()),
            ..Settings::default()
        })
        .subscription(subscription_window_resize)
        .run()
}
