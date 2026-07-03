use iced::widget::{button, column, text};

use crate::{
    app::{Component, Message},
    screens::Screen,
};

pub fn render_main_screen() -> Component {
    column![
        text!("main menu"),
        button("Game Screen").on_press(Message::ChangeScreen(Screen::Game)),
        button("Counter").on_press(Message::ChangeScreen(Screen::DebugCounter)),
        button("Tile Debug").on_press(Message::ChangeScreen(Screen::DebugTile)),
        button("Settings").on_press(Message::ChangeScreen(Screen::Settings)),
    ]
    .into()
}
