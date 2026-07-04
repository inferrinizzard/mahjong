use iced::widget::pin;
use mahjong_lib::{Meld, consts::Wind};

use crate::{
    app::{
        Component,
        game::GameTile,
        render::{
            consts::{Direction, TILE_ASPECT_RATIO, get_total_tile_length},
            render_tile::render_hand,
        },
        settings::Settings,
    },
    util::PositionTuple,
};

#[derive(Default)]
pub struct Player {
    pub hand: Vec<GameTile>,
    pub open_melds: Vec<Meld>,
    pub discard: Vec<GameTile>,
    pub active_tile: Option<GameTile>,

    pub wind: Wind,
    pub position: Direction,
}

impl Player {
    pub fn new(position: Direction, wind: Wind) -> Self {
        Self {
            hand: vec![],
            open_melds: vec![],
            discard: vec![],
            active_tile: None,

            wind,
            position,
        }
    }

    pub fn add_tiles(&mut self, tiles: Vec<GameTile>) {
        self.hand.extend(tiles);
    }

    pub fn render(&self, settings: &Settings) -> Vec<Component> {
        // for each player:
        // - hand
        // - active tile ?
        // - discards
        // - open melds

        let (hand_x, hand_y) =
            get_position_for_player_hand(settings, &self.position, self.hand.len());

        vec![
            pin(render_hand(
                &self.hand,
                settings.video.tile_size,
                &self.position,
            ))
            .x(hand_x)
            .y(hand_y)
            .into(),
        ]
    }
}

fn get_position_for_player_hand(
    settings: &Settings,
    direction: &Direction,
    num_tiles: usize,
) -> PositionTuple {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;

    match direction {
        Direction::DOWN => (
            window_size.width / 2. - get_total_tile_length(num_tiles, tile_size) / 2.,
            window_size.height - tile_size as f32 * TILE_ASPECT_RATIO,
        ),
        Direction::RIGHT => (
            window_size.width - tile_size as f32 * TILE_ASPECT_RATIO,
            window_size.height / 2. - get_total_tile_length(num_tiles, tile_size) / 2.,
        ),
        Direction::UP => (
            window_size.width / 2. - get_total_tile_length(num_tiles, tile_size) / 2.,
            0.,
        ),
        Direction::LEFT => (
            0.,
            window_size.height / 2. - get_total_tile_length(num_tiles, tile_size) / 2.,
        ),
    }
}
