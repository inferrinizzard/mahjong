use iced::widget::{button, container, pin, text};

use crate::{
    app::{
        Component, Message,
        game::{GameMessage, deck::Deck},
        render::{
            consts::{Direction, TILE_ASPECT_RATIO, TILE_EDGE_RATIO, get_total_tile_length},
            render_tile::render_bank,
        },
        settings::Settings,
    },
    util::debug_outline::debug_outline,
};

pub fn render_deck(deck: &Deck, settings: &Settings) -> Vec<Component> {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;

    let center = (window_size.width / 2., window_size.height / 2.);
    let center_side = tile_size as f32 * 5.;

    let bank_length =
        get_total_tile_length(deck.bank_size / 2, tile_size) + tile_size as f32 * TILE_EDGE_RATIO;
    let bank_depth = tile_size as f32 * (TILE_ASPECT_RATIO + TILE_EDGE_RATIO);

    vec![
        pin(debug_outline(
            container(text(deck.count))
                .width(center_side * 2.)
                .height(center_side * 2.)
                .into(),
        ))
        .x(center.0 - center_side)
        .y(center.1 - center_side)
        .into(),
        pin(render_bank(&deck.banks[0], tile_size, Direction::DOWN))
            .x(center.0 - center_side)
            .y(center.1 + center_side)
            .into(),
        pin(render_bank(&deck.banks[1], tile_size, Direction::RIGHT))
            .x(center.0 + center_side)
            .y(center.1 + center_side - bank_length + (tile_size as f32) * TILE_EDGE_RATIO)
            .into(),
        pin(render_bank(&deck.banks[3], tile_size, Direction::LEFT))
            .x(center.0 - center_side - bank_depth)
            .y(center.1 - center_side)
            .into(),
        pin(render_bank(&deck.banks[2], tile_size, Direction::UP))
            .x(center.0 + center_side - bank_length + (tile_size as f32) * TILE_EDGE_RATIO)
            .y(center.1 - center_side - bank_depth)
            .into(),
    ]
}

pub fn render_buttons(settings: &Settings) -> Vec<Component> {
    let tile_size = settings.video.tile_size;
    let window_size = settings.video.window_size;
    let center = (window_size.width / 2., window_size.height / 2.);

    let sort_button = button("Sort").on_press(Message::Game(GameMessage::SortHand));

    vec![
        pin(sort_button)
            .x(center.0 - (tile_size as f32) * 3.)
            .y(window_size.height - (tile_size as f32) * TILE_ASPECT_RATIO * 2.)
            .into(),
    ]
}
