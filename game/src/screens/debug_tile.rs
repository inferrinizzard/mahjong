use iced::{
    Element,
    widget::{Stack, button, column, container, pin, svg},
};

use crate::{app::Message, screens::Screen, util::get_path::get_path};

pub struct DebugTile {}

impl Default for DebugTile {
    fn default() -> Self {
        DebugTile {}
    }
}

impl DebugTile {
    pub fn view(&self) -> Element<'static, Message> {
        column![
            button("Back to Main").on_press(Message::ChangeScreen(Screen::Main)),
            render_tile_row(vec!["1m", "2m"], 128),
            render_tile_row(
                vec!["1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m"],
                64
            ),
        ]
        .into()
    }
}

fn render_tile_row(tiles: Vec<&str>, size: u32) -> Element<'static, Message> {
    let tile_face_width_coeffecient = size as f32 * 50. / 64.;

    let mut row_stack_vec: Vec<Element<Message>> = vec![];
    for (i, tile) in tiles.iter().enumerate() {
        row_stack_vec.push(
            pin(container(
                svg(get_path(
                    format!("assets/tiles/oblique/suits/{}.svg", tile).as_str(),
                ))
                .width(size),
            ))
            .x(i as f32 * tile_face_width_coeffecient)
            .into(),
        );
    }

    Stack::from_vec(row_stack_vec)
        .width(tile_face_width_coeffecient * tiles.len() as f32 + size as f32)
        .into()
}
