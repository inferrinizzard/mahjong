use crate::{
    game::{player::Player, Deck},
    structs::{Hand, Tile},
};

const NUM_BASE_TILES: usize = 13;

pub fn create_players(deck: &mut Deck, names: Vec<String>) -> Vec<Player> {
    let mut tile_vecs = names
        .iter()
        .map(|_| vec![] as Vec<Tile>)
        .collect::<Vec<Vec<Tile>>>();

    for i in 0..tile_vecs.len() * NUM_BASE_TILES {
        let hand_index = i % tile_vecs.len();
        let tile = deck.pop_back().unwrap();
        tile_vecs[hand_index].push(tile)
    }

    names
        .iter()
        .map(|name| {
            let hand = Hand::new(tile_vecs.pop().unwrap());
            Player::new(name.to_owned(), hand)
        })
        .collect::<Vec<Player>>()
}
