use iced::widget::pin;
use mahjong_lib::{Meld, consts::Wind};

use crate::{
    app::{
        Component,
        game::GameTile,
        render::{
            consts::{Direction, TILE_ASPECT_RATIO, get_total_tile_length},
            render_tile::{render_hand, render_tile},
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

    pub fn play_tile(&mut self, id: String) {
        // Play active tile
        if let Some(active_tile) = &self.active_tile
            && active_tile.id == id
        {
            self.discard.push(self.active_tile.take().unwrap());
            self.active_tile = None;
        }
        // Play tile from hand
        else {
            if let Some(index) = self.hand.iter().position(|tile| tile.id == id) {
                let target_hand_tile = self.hand.remove(index);
                self.discard.push(target_hand_tile);
                self.hand.push(self.active_tile.take().unwrap());
            }
        }
    }

    pub fn render(&self, settings: &Settings) -> Vec<Component> {
        // for each player:
        // - hand
        // - active tile ?
        // - discards
        // - open melds

        let PlayerRenderPositionStruct { hand, active } =
            get_player_positions(settings, &self.position, self.hand.len());

        let mut components = vec![
            pin(render_hand(
                &self.hand,
                settings.video.tile_size,
                &self.position,
            ))
            .x(hand.0)
            .y(hand.1)
            .into(),
        ];

        if self.active_tile.is_some() {
            components.push(render_tile(
                self.active_tile.as_ref().unwrap(),
                settings.video.tile_size,
                &self.position,
                (active.0, active.1),
            ));
        }

        components
    }
}

struct PlayerRenderPositionStruct {
    hand: PositionTuple,
    active: PositionTuple,
    // open: PositionTuple,
    // discard: PositionTuple,
}

fn get_player_positions(
    settings: &Settings,
    direction: &Direction,
    num_tiles: usize,
) -> PlayerRenderPositionStruct {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;
    let total_tile_length = get_total_tile_length(num_tiles, tile_size);

    let hand;
    let active;

    match direction {
        Direction::DOWN => {
            let x = window_size.width / 2. - total_tile_length / 2.;
            let y = window_size.height - tile_size as f32 * TILE_ASPECT_RATIO;

            hand = (x, y);
            active = (x + total_tile_length + (tile_size as f32) / 2., y);
        }
        Direction::RIGHT => {
            let x = window_size.width - tile_size as f32 * TILE_ASPECT_RATIO;
            let y = window_size.height / 2. - total_tile_length / 2.;

            hand = (x, y);
            active = (x, y - (tile_size as f32) * 1.5);
        }
        Direction::UP => {
            let x = window_size.width / 2. - total_tile_length / 2.;
            let y = 0.;

            hand = (x, y);
            active = (x - (tile_size as f32) * 1.5, y);
        }
        Direction::LEFT => {
            let x = 0.;
            let y = window_size.height / 2. - total_tile_length / 2.;

            hand = (x, y);
            active = (x, y + total_tile_length + (tile_size as f32) / 2.);
        }
    }

    PlayerRenderPositionStruct { hand, active }
}
