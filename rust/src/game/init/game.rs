use strum::IntoEnumIterator;

use crate::{
    consts::Wind,
    game::{create_deck, create_players, Deck, GameOptions, Player},
};

pub fn create_game(game_options: &GameOptions) -> (Deck, Vec<Player>) {
    // Create Deck
    let mut deck = create_deck(&game_options.deck);

    let names = vec!["A", "B", "C", "D"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    // Create players with starting hands
    let mut players = create_players(&mut deck, names);

    // Assign winds for players
    let first_dealer_index = rand::random_range(0..players.len());
    for i in 0..players.len() {
        let index = (i + first_dealer_index) % players.len();
        players[index].wind = Wind::iter().nth(i).unwrap();
    }

    (deck, players)
}
