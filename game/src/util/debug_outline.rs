use iced::{
    Border, Color, Element,
    border::Radius,
    widget::{self, container::Style},
};

use crate::app::Message;

pub fn debug_outline(element: Element<'static, Message>) -> Element<'static, Message> {
    widget::container(element)
        .style(|_s| Style {
            border: Border {
                color: Color {
                    r: 1.,
                    g: 0.,
                    b: 0.,
                    a: 0.5,
                },
                width: 1.,
                radius: Radius::default(),
            },
            ..Style::default()
        })
        .into()
}
