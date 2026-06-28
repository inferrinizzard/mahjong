use iced::{Element, widget::pin};

use crate::app::{
    Message,
    game::Game,
    render::{
        consts::{TILE_ASPECT_RATIO, get_total_tile_length},
        render_tile::render_hand,
    },
    settings::Settings,
};

// pub fn window_layout_stack)_

pub fn render_game_hands(game: &Game, settings: &Settings) -> Vec<Element<'static, Message>> {
    vec![
        pin(render_hand(&game.hands[&0], settings.video.tile_size))
            .x(settings.video.window_size.width / 2.
                - get_total_tile_length(game.hands[&0].len(), settings.video.tile_size) / 2.)
            .y(settings.video.window_size.height
                - settings.video.tile_size as f32 * TILE_ASPECT_RATIO)
            .into(),
        pin(render_hand(&game.hands[&2], settings.video.tile_size))
            .x(settings.video.window_size.width / 2.
                - get_total_tile_length(game.hands[&0].len(), settings.video.tile_size) / 2.)
            .y(0.)
            .into(),
    ]
}
