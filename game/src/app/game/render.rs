use iced::{Element, widget::pin};

use crate::app::{
    Message,
    game::Game,
    render::{
        consts::{Direction, TILE_ASPECT_RATIO, get_total_tile_length},
        render_tile::render_hand,
    },
    settings::Settings,
};

pub fn render_game_hands(game: &Game, settings: &Settings) -> Vec<Element<'static, Message>> {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;

    vec![
        pin(render_hand(&game.hands[&0], tile_size, Direction::DOWN))
            .x(
                window_size.width / 2.
                    - get_total_tile_length(game.hands[&0].len(), tile_size) / 2.,
            )
            .y(window_size.height - tile_size as f32 * TILE_ASPECT_RATIO)
            .into(),
        pin(render_hand(&game.hands[&1], tile_size, Direction::RIGHT))
            .x(window_size.width - tile_size as f32 * TILE_ASPECT_RATIO)
            .y(window_size.height / 2.
                - get_total_tile_length(game.hands[&0].len(), tile_size) / 2.)
            .into(),
        pin(render_hand(&game.hands[&2], tile_size, Direction::UP))
            .x(
                window_size.width / 2.
                    - get_total_tile_length(game.hands[&0].len(), tile_size) / 2.,
            )
            .y(0.)
            .into(),
        pin(render_hand(&game.hands[&3], tile_size, Direction::LEFT))
            .x(0.)
            .y(window_size.height / 2.
                - get_total_tile_length(game.hands[&0].len(), tile_size) / 2.)
            .into(),
    ]
}
