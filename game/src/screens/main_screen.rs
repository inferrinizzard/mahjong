use iced::widget::{Column, button, column, text};

use crate::app::root::{Message, Screen};

pub fn render_main_screen() -> Column<'static, Message> {
    column![
        text!("main menu"),
        button("Counter").on_press(Message::ChangeScreen(Screen::DebugCounter)),
    ]
}
