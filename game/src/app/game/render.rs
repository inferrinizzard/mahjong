use iced::{
    Element,
    widget::{container, pin, text},
};
use mahjong_lib::consts::Wind;

use crate::{
    app::{
        Message,
        game::Game,
        render::{
            consts::{Direction, TILE_ASPECT_RATIO, TILE_EDGE_RATIO, get_total_tile_length},
            render_tile::{render_bank, render_hand},
        },
        settings::Settings,
    },
    util::debug_outline::debug_outline,
};

pub fn render_game_hands(game: &Game, settings: &Settings) -> Vec<Element<'static, Message>> {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;

    vec![
        pin(render_hand(
            &game.players[&Wind::EAST].hand,
            tile_size,
            Direction::DOWN,
        ))
        .x(window_size.width / 2.
            - get_total_tile_length(game.players[&Wind::EAST].hand.len(), tile_size) / 2.)
        .y(window_size.height - tile_size as f32 * TILE_ASPECT_RATIO)
        .into(),
        pin(render_hand(
            &game.players[&Wind::NORTH].hand,
            tile_size,
            Direction::RIGHT,
        ))
        .x(window_size.width - tile_size as f32 * TILE_ASPECT_RATIO)
        .y(window_size.height / 2.
            - get_total_tile_length(game.players[&Wind::NORTH].hand.len(), tile_size) / 2.)
        .into(),
        pin(render_hand(
            &game.players[&Wind::WEST].hand,
            tile_size,
            Direction::UP,
        ))
        .x(window_size.width / 2.
            - get_total_tile_length(game.players[&Wind::WEST].hand.len(), tile_size) / 2.)
        .y(0.)
        .into(),
        pin(render_hand(
            &game.players[&Wind::SOUTH].hand,
            tile_size,
            Direction::LEFT,
        ))
        .x(0.)
        .y(window_size.height / 2.
            - get_total_tile_length(game.players[&Wind::SOUTH].hand.len(), tile_size) / 2.)
        .into(),
    ]
}

pub fn render_game_banks(game: &Game, settings: &Settings) -> Vec<Element<'static, Message>> {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;

    let center = (window_size.width / 2., window_size.height / 2.);
    let center_side = tile_size as f32 * 5.;

    let bank_length = get_total_tile_length(game.deck.bank_size / 2, tile_size)
        + tile_size as f32 * TILE_EDGE_RATIO;
    let bank_depth = tile_size as f32 * (TILE_ASPECT_RATIO + TILE_EDGE_RATIO);

    vec![
        pin(debug_outline(
            container(text(game.deck.count))
                .width(center_side * 2.)
                .height(center_side * 2.)
                .into(),
        ))
        .x(center.0 - center_side)
        .y(center.1 - center_side)
        .into(),
        pin(render_bank(game.deck.bank_size, tile_size, Direction::DOWN))
            .x(center.0 - center_side)
            .y(center.1 + center_side)
            .into(),
        pin(render_bank(
            game.deck.bank_size,
            tile_size,
            Direction::RIGHT,
        ))
        .x(center.0 + center_side)
        .y(center.1 + center_side - bank_length + (tile_size as f32) * TILE_EDGE_RATIO)
        .into(),
        pin(render_bank(game.deck.bank_size, tile_size, Direction::LEFT))
            .x(center.0 - center_side - bank_depth)
            .y(center.1 - center_side)
            .into(),
        pin(render_bank(game.deck.bank_size, tile_size, Direction::UP))
            .x(center.0 + center_side - bank_length + (tile_size as f32) * TILE_EDGE_RATIO)
            .y(center.1 - center_side - bank_depth)
            .into(),
    ]
}
