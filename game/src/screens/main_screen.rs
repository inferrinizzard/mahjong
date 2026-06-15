use iced::{
    Element,
    widget::{button, column, text},
};

use crate::{app::Message, screens::Screen};

pub fn render_main_screen() -> Element<'static, Message> {
    column![
        text!("main menu"),
        button("Counter").on_press(Message::ChangeScreen(Screen::DebugCounter)),
        button("Tile Debug").on_press(Message::ChangeScreen(Screen::DebugTile)),
        button("Settings").on_press(Message::ChangeScreen(Screen::Settings)),
    ]
    .into()
}
