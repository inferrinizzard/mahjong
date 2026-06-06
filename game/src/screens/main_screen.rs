use iced::{Element, widget::text};

use crate::app::root::Message;

pub fn render_main_screen() -> Element<'static, Message> {
    text!("main").into()
}
