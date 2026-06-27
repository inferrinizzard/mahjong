use iced::{
    Element,
    widget::{Stack, container, pin, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{
    app::{
        Message,
        render::consts::{TILE_BACK_PATH, TILE_FACE_RATIO},
    },
    util::get_path::get_path,
};

pub fn render_tile_for_path(tile_path: &str, size: u32) -> Element<'static, Message> {
    container(svg(tile_path).width(size)).into()
}

pub fn render_tile(tile: &TileData, size: u32) -> Element<'static, Message> {
    let tile_dir = match tile.suit {
        Suit::BAMBOO | Suit::MAN | Suit::TONG => "suits",
        Suit::DRAGON => "dragon",
        Suit::WIND => "wind",
        Suit::JOKER => "joker",
        Suit::FLOWER | Suit::SEASON => "bonus",
    };

    let lower_tile_name = tile.name.to_lowercase();
    let file_name = match tile.suit {
        Suit::BAMBOO | Suit::MAN | Suit::TONG => tile.code,
        Suit::DRAGON => "dragon",
        Suit::WIND => &lower_tile_name.split("_").take(1).collect::<Vec<&str>>()[0],
        Suit::JOKER => "joker_baida",
        Suit::FLOWER | Suit::SEASON => {
            let slugs = &mut lower_tile_name.split("_").collect::<Vec<&str>>();
            slugs.reverse();
            slugs.join("_").leak()
        }
    };

    let tile_path =
        get_path(format!("assets/tiles/oblique/{}/{}.svg", tile_dir, file_name).as_str());
    render_tile_for_path(&tile_path, size)
}

pub fn render_bank(num_tiles: usize, size: u32) -> Element<'static, Message> {
    let tile_face_length = size as f32 * TILE_FACE_RATIO;

    let mut row_stack_vec: Vec<Element<Message>> = vec![];
    for i in 0..num_tiles {
        let is_top_tile = i % 2 == 0;
        let mut x = (i / 2) as f32 * tile_face_length;
        let mut y = 0.;

        if is_top_tile {
            x += size as f32 - tile_face_length
        } else {
            y += size as f32 - tile_face_length
        }

        row_stack_vec.push(
            pin(render_tile_for_path(&get_path(TILE_BACK_PATH), size))
                .x(x)
                .y(y)
                .into(),
        );
    }

    Stack::from_vec(row_stack_vec)
        .width(tile_face_length * num_tiles as f32 + size as f32)
        .height(size * 2)
        .into()
}
