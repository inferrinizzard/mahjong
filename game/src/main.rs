pub mod app;
pub mod screens;
pub mod util;

use crate::app::root::AppRoot;

pub fn main() -> iced::Result {
    iced::application(AppRoot::default, AppRoot::update, AppRoot::view)
        .title("TEST")
        .run()
}
