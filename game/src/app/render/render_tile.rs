use iced::{Element, widget::Stack};

use crate::app::{
    Message,
    game::GameTile,
    render::{
        consts::{Direction, TILE_ASPECT_RATIO, TILE_EDGE_RATIO, get_total_tile_length},
        tile_renderer::TileRenderer,
    },
};

use super::consts::TILE_FACE_RATIO;

pub fn render_tile(tile: &GameTile, size: u32) -> Element<'static, Message> {
    let mut renderer = TileRenderer::new();
    renderer.set_size(size as f32);
    renderer.set_tile(tile);

    renderer.render()
}

pub fn render_hand(
    tiles: &Vec<GameTile>,
    size: u32,
    direction: Direction,
) -> Element<'static, Message> {
    render_tileset(
        tiles
            .iter()
            .map(|tile| {
                let mut renderer = TileRenderer::new();
                renderer.set_size(size as f32);
                renderer.set_direction(&direction);
                renderer.set_tile(tile);

                Some(renderer)
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
            if matches!(tile, None) {
                return None;
            }

            let mut renderer = TileRenderer::new();
            renderer.set_size(size as f32);
            renderer.set_direction(&direction);

            Some(renderer)
        })
        .collect();

    render_tileset(paths, size, &direction, 2)
}

pub struct PathItem {
    pub path: String,
    pub id: String,
}

pub fn render_tileset(
    _paths: Vec<Option<TileRenderer>>,
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
    for (i, _renderer) in paths.iter_mut().enumerate() {
        if matches!(_renderer, None) {
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

        let mut renderer = _renderer.take().unwrap();
        renderer.set_position(x, y);

        stack_vec.push(renderer.render());
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
