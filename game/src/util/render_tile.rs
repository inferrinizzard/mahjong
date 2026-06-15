use iced::{
    Element,
    widget::{container, svg},
};
use mahjong_lib::tile::TileData;

use crate::{app::Message, util::get_path::get_path};

pub fn render_tile(tile: &TileData, size: u32) -> Element<'static, Message> {
    let tile_path = get_path(format!("assets/tiles/oblique/suits/{}.svg", tile.code).as_str());
    container(svg(tile_path).width(size)).into()
}
