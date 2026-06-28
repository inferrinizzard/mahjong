use std::path::Path;

use iced::{
    Element,
    widget::{Stack, container, pin, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{
    app::{
        Message,
        render::consts::{Direction, TILE_ASPECT_RATIO, TILE_EDGE_RATIO},
    },
    util::get_path::get_path,
};

use super::consts::{TILE_BACK_PATH, TILE_FACE_RATIO};

pub fn render_tile_for_path(tile_path: &str, size: u32) -> Element<'static, Message> {
    container(svg(tile_path).width(size)).into()
}

pub fn get_path_for_tile(tile: &TileData, direction: &Direction) -> String {
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

    let suffix = match direction {
        Direction::DOWN => "",
        Direction::RIGHT => "_right",
        Direction::UP => "_up",
        Direction::LEFT => "_left",
    };

    let mut path = get_path(
        format!(
            "assets/tiles/oblique/{}/{}{}.svg",
            tile_dir, file_name, suffix
        )
        .as_str(),
    );

    if !Path::new(&path).exists() {
        match direction {
            Direction::LEFT => path = path.replace("_left", "_x"),
            Direction::RIGHT => path = path.replace("_right", "_x"),
            Direction::UP => path = path.replace("_up", ""),
            _ => {}
        }
    }

    path
}

pub fn render_tile(tile: &TileData, size: u32) -> Element<'static, Message> {
    render_tile_for_path(get_path_for_tile(tile, &Direction::DOWN).as_str(), size)
}

pub fn render_hand(
    tiles: &Vec<TileData>,
    size: u32,
    direction: Direction,
) -> Element<'static, Message> {
    render_tileset(
        tiles
            .iter()
            .map(|tile| get_path_for_tile(tile, &direction))
            .collect(),
        size,
        &direction,
        false,
    )
}

pub fn render_bank(num_tiles: usize, size: u32, direction: Direction) -> Element<'static, Message> {
    render_tileset(
        vec![TILE_BACK_PATH.to_string(); num_tiles],
        size,
        &direction,
        true,
    )
}

pub fn render_tileset(
    paths: Vec<String>,
    size: u32,
    direction: &Direction,
    should_stack: bool,
) -> Element<'static, Message> {
    let tile_face_length = size as f32 * TILE_FACE_RATIO;
    let is_vertical = matches!(direction, Direction::LEFT | Direction::RIGHT);

    let mut stack_vec: Vec<Element<Message>> = vec![];
    for (i, path) in paths.iter().enumerate() {
        let is_top_tile = if should_stack { i % 2 == 0 } else { true };
        let mut x = (if should_stack { i / 2 } else { i }) as f32 * tile_face_length;
        let mut y = 0.;

        if is_top_tile {
            x += size as f32 - tile_face_length
        } else {
            y += size as f32 - tile_face_length
        }

        if is_vertical {
            let z = x;
            x = y;
            y = z;
        }

        stack_vec.push(
            pin(render_tile_for_path(&get_path(path), size))
                .x(x)
                .y(y)
                .into(),
        );
    }

    let container_width = if is_vertical {
        size as f32 * (TILE_ASPECT_RATIO + TILE_EDGE_RATIO)
    } else {
        tile_face_length * paths.len() as f32 + size as f32
    };
    let container_height = if is_vertical {
        tile_face_length * paths.len() as f32 + size as f32
    } else {
        size as f32 * (TILE_ASPECT_RATIO + TILE_EDGE_RATIO)
    };

    Stack::from_vec(stack_vec)
        .width(container_width)
        .height(container_height)
        .into()
}
