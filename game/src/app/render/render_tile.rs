use iced::{
    Element,
    widget::{Stack, container, pin, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{app::Message, util::get_path::get_path};

use super::consts::{TILE_BACK_PATH, TILE_FACE_RATIO};

pub fn render_tile_for_path(tile_path: &str, size: u32) -> Element<'static, Message> {
    container(svg(tile_path).width(size)).into()
}

pub fn get_path_for_tile(tile: &TileData) -> String {
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
        Suit::DRAGON => match tile.name {
            "GREEN_DRAGON" => "fa",
            "RED_DRAGON" => "zhong",
            "WHITE_DRAGON" => "ban",
            _ => "",
        },
        Suit::WIND => &lower_tile_name.split("_").take(1).collect::<Vec<&str>>()[0],
        Suit::JOKER => "joker_baida",
        Suit::FLOWER | Suit::SEASON => {
            let slugs = &mut lower_tile_name.split("_").collect::<Vec<&str>>();
            slugs.reverse();
            slugs.join("_").leak()
        }
    };

    get_path(format!("assets/tiles/oblique/{}/{}.svg", tile_dir, file_name).as_str())
}

pub fn render_tile(tile: &TileData, size: u32) -> Element<'static, Message> {
    render_tile_for_path(get_path_for_tile(tile).as_str(), size)
}

pub fn render_hand(tiles: &Vec<TileData>, size: u32) -> Element<'static, Message> {
    render_tileset(
        tiles.iter().map(|tile| get_path_for_tile(tile)).collect(),
        size,
        false,
    )
}

pub fn render_bank(num_tiles: usize, size: u32) -> Element<'static, Message> {
    render_tileset(vec![TILE_BACK_PATH.to_string(); num_tiles], size, true)
}

pub fn render_tileset(
    paths: Vec<String>,
    size: u32,
    should_stack: bool,
) -> Element<'static, Message> {
    let tile_face_length = size as f32 * TILE_FACE_RATIO;

    let mut row_stack_vec: Vec<Element<Message>> = vec![];
    for (i, path) in paths.iter().enumerate() {
        let is_top_tile = if should_stack { i % 2 == 0 } else { true };
        let mut x = (if should_stack { i / 2 } else { i }) as f32 * tile_face_length;
        let mut y = 0.;

        if is_top_tile {
            x += size as f32 - tile_face_length
        } else {
            y += size as f32 - tile_face_length
        }

        row_stack_vec.push(
            pin(render_tile_for_path(&get_path(path), size))
                .x(x)
                .y(y)
                .into(),
        );
    }

    Stack::from_vec(row_stack_vec)
        .width(tile_face_length * paths.len() as f32 + size as f32)
        .height(size * 2)
        .into()
}
