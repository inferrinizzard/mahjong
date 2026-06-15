use iced::{
    Element,
    widget::{Stack, button, column, container, pin, stack, svg},
};

use crate::{app::root::Message, screens::Screen, util::get_path::get_path};

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
            stack![
                pin(container(
                    svg(get_path("assets/tiles/oblique/suits/1m.svg")).width(128)
                ))
                .x(0),
                pin(container(
                    svg(get_path("assets/tiles/oblique/suits/2m.svg")).width(128)
                ))
                .x(100),
                // .width(138)
                // .padding(Padding {
                //     left: 10.,
                //     top: 0.,
                //     bottom: 0.,
                //     right: 0.
                // })
                // .style(|s| {
                //     container::Style {
                //         border: Border {
                //             color: Color {
                //                 r: 1.,
                //                 g: 0.,
                //                 b: 0.,
                //                 a: 0.5,
                //             },
                //             width: 1.,
                //             radius: Radius {
                //                 top_left: 0.,
                //                 top_right: 0.,
                //                 bottom_left: 0.,
                //                 bottom_right: 0.,
                //             },
                //         },
                //         background: None,
                //         shadow: Shadow {
                //             blur_radius: 0.,
                //             color: Color {
                //                 r: 0.,
                //                 g: 0.,
                //                 b: 0.,
                //                 a: 0.,
                //             },
                //             offset: Vector { x: 0., y: 0. },
                //         },
                //         snap: false,
                //         text_color: None,
                //     }
                // }),
                // container(svg(get_path("assets/tiles/oblique/suits/3m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/4m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/5m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/6m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/7m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/8m.svg"))),
                // container(svg(get_path("assets/tiles/oblique/suits/9m.svg"))),
            ],
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
