use crate::game::{create_deck, init::player::create_players, player::Player, Deck, DeckFlags};

pub fn create_game() -> (Deck, Vec<Player>) {
    let mut deck = create_deck(DeckFlags {
        has_akadora: true,
        has_flowers: true,
        has_seasons: true,
    });

    let names = vec!["A", "B", "C", "D"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let players = create_players(&mut deck, names);

    (deck, players)
}
