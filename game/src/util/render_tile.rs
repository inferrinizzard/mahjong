use iced::{
    Element,
    widget::{container, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{app::Message, util::get_path::get_path};

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
    container(svg(tile_path).width(size)).into()
}
