use iced::{
    Element,
    widget::{button, column, text},
};

use crate::app::{render::Screen, root::Message};

pub fn render_main_screen() -> Element<'static, Message> {
    column![
        text!("main menu"),
        button("Counter").on_press(Message::ChangeScreen(Screen::DebugCounter)),
        button("Tile Debug").on_press(Message::ChangeScreen(Screen::DebugTile)),
    ]
    .into()
}
