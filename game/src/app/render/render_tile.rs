use std::path::Path;

use iced::{
    Element,
    widget::{Stack, container, mouse_area, pin, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{
    app::{
        Message,
        game::{GameMessage, GameTile},
        render::consts::{
            Direction, TILE_ASPECT_RATIO, TILE_BACK_X_PATH, TILE_EDGE_RATIO, get_total_tile_length,
        },
    },
    util::get_path::get_path,
};

use super::consts::{TILE_BACK_PATH, TILE_FACE_RATIO};

pub fn render_tile_for_path(
    tile_path: &str,
    size: u32,
    direction: &Direction,
) -> Element<'static, Message> {
    if !Path::new(tile_path).exists() {
        println!("can't find tile: {}", tile_path);
    }

    let mut svg = svg(tile_path);
    if matches!(direction, Direction::DOWN | Direction::UP) {
        svg = svg.width(size)
    } else {
        svg = svg.height(size)
    }
    container(svg).into()
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
            slugs.join("-").leak()
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
    render_tile_for_path(
        get_path_for_tile(tile, &Direction::DOWN).as_str(),
        size,
        &Direction::DOWN,
    )
}

pub fn render_hand(
    tiles: &Vec<GameTile>,
    size: u32,
    direction: Direction,
) -> Element<'static, Message> {
    render_tileset(
        tiles
            .iter()
            .map(|tile| PathItem {
                path: get_path_for_tile(&tile.data, &direction),
                id: tile.id.clone(),
            })
            .collect(),
        size,
        &direction,
        1,
    )
}

pub fn render_bank(
    bank_tile_displays: &Vec<Option<GameTile>>,
    size: u32,
    direction: Direction,
) -> Element<'static, Message> {
    let paths = bank_tile_displays
        .iter()
        .map(|tile| {
            let path_str;
            if matches!(tile, None) {
                path_str = "";
            } else if matches!(direction, Direction::DOWN | Direction::UP) {
                path_str = TILE_BACK_PATH
            } else {
                path_str = TILE_BACK_X_PATH
            }

            PathItem {
                path: path_str.to_string(),
                id: String::new(),
            }
        })
        .collect();

    render_tileset(paths, size, &direction, 2)
}

pub struct PathItem {
    pub path: String,
    pub id: String,
}

pub fn render_tileset(
    _paths: Vec<PathItem>,
    size: u32,
    direction: &Direction,
    num_rows: usize,
) -> Element<'static, Message> {
    let mut paths = _paths;
    if matches!(direction, Direction::LEFT | Direction::UP) {
        paths.reverse();
    }

    let tile_face_length = size as f32 * TILE_FACE_RATIO;
    let is_vertical = matches!(direction, Direction::LEFT | Direction::RIGHT);
    let total_length = get_total_tile_length((paths.len() + num_rows - 1) / num_rows, size);

    let mut stack_vec: Vec<Element<Message>> = vec![];
    for (i, PathItem { path, id }) in paths.iter().enumerate() {
        if path.is_empty() {
            continue;
        }

        // position based on index
        let mut x = (i / num_rows) as f32 * tile_face_length;
        let mut y = 0.;

        // offset rows for stacking
        x += size as f32 * TILE_EDGE_RATIO * (num_rows - (i % num_rows) - 1) as f32;
        y += size as f32
            * TILE_EDGE_RATIO
            * if is_vertical {
                num_rows - (i % num_rows) - 1
            } else {
                i % num_rows
            } as f32;

        if is_vertical {
            let z = x;
            x = y;
            y = total_length - z - size as f32;
        }

        let mut tile_element = render_tile_for_path(&get_path(path), size, direction).into();

        // add click handler if id is present (render hand)
        if !id.is_empty() {
            let message = Message::Game(GameMessage::TileClick(id.to_owned(), direction.clone()));
            tile_element = mouse_area(tile_element).on_press(message).into()
        }
        tile_element = pin(tile_element).x(x).y(y).into();

        stack_vec.push(tile_element);
    }

    let long_length = total_length + (num_rows - 1) as f32 * TILE_EDGE_RATIO * size as f32;
    let short_length = size as f32 * (TILE_ASPECT_RATIO + (num_rows - 1) as f32 * TILE_EDGE_RATIO);

    let container_width = if is_vertical {
        short_length
    } else {
        long_length
    };
    let container_height = if is_vertical {
        long_length
    } else {
        short_length
    };

    Stack::from_vec(stack_vec)
        .width(container_width)
        .height(container_height)
        .into()
}
