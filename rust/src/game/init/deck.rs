use std::collections::VecDeque;

use crate::{
    consts::{tile::TileNumber, Dragon, Flower, Season, TileData, Wind},
    structs::Tile,
};

pub type Deck = VecDeque<Tile>;

pub struct DeckFlags {
    pub has_flowers: bool,
    pub has_seasons: bool,
    pub has_akadora: bool,
}

pub fn create_deck(flags: DeckFlags) -> Deck {
    let mut base_deck = VecDeque::from(vec![
        Tile::new(TileData::MAN(TileNumber::ONE)),
        Tile::new(TileData::MAN(TileNumber::ONE)),
        Tile::new(TileData::MAN(TileNumber::ONE)),
        Tile::new(TileData::MAN(TileNumber::ONE)),
        Tile::new(TileData::MAN(TileNumber::TWO)),
        Tile::new(TileData::MAN(TileNumber::TWO)),
        Tile::new(TileData::MAN(TileNumber::TWO)),
        Tile::new(TileData::MAN(TileNumber::TWO)),
        Tile::new(TileData::MAN(TileNumber::THREE)),
        Tile::new(TileData::MAN(TileNumber::THREE)),
        Tile::new(TileData::MAN(TileNumber::THREE)),
        Tile::new(TileData::MAN(TileNumber::THREE)),
        Tile::new(TileData::MAN(TileNumber::FOUR)),
        Tile::new(TileData::MAN(TileNumber::FOUR)),
        Tile::new(TileData::MAN(TileNumber::FOUR)),
        Tile::new(TileData::MAN(TileNumber::FOUR)),
        // Skip one copy of MAN_5
        Tile::new(TileData::MAN(TileNumber::FIVE)),
        Tile::new(TileData::MAN(TileNumber::FIVE)),
        Tile::new(TileData::MAN(TileNumber::FIVE)),
        Tile::new(TileData::MAN(TileNumber::SIX)),
        Tile::new(TileData::MAN(TileNumber::SIX)),
        Tile::new(TileData::MAN(TileNumber::SIX)),
        Tile::new(TileData::MAN(TileNumber::SIX)),
        Tile::new(TileData::MAN(TileNumber::SEVEN)),
        Tile::new(TileData::MAN(TileNumber::SEVEN)),
        Tile::new(TileData::MAN(TileNumber::SEVEN)),
        Tile::new(TileData::MAN(TileNumber::SEVEN)),
        Tile::new(TileData::MAN(TileNumber::EIGHT)),
        Tile::new(TileData::MAN(TileNumber::EIGHT)),
        Tile::new(TileData::MAN(TileNumber::EIGHT)),
        Tile::new(TileData::MAN(TileNumber::EIGHT)),
        Tile::new(TileData::MAN(TileNumber::NINE)),
        Tile::new(TileData::MAN(TileNumber::NINE)),
        Tile::new(TileData::MAN(TileNumber::NINE)),
        Tile::new(TileData::MAN(TileNumber::NINE)),
        Tile::new(TileData::TONG(TileNumber::ONE)),
        Tile::new(TileData::TONG(TileNumber::ONE)),
        Tile::new(TileData::TONG(TileNumber::ONE)),
        Tile::new(TileData::TONG(TileNumber::ONE)),
        Tile::new(TileData::TONG(TileNumber::TWO)),
        Tile::new(TileData::TONG(TileNumber::TWO)),
        Tile::new(TileData::TONG(TileNumber::TWO)),
        Tile::new(TileData::TONG(TileNumber::TWO)),
        Tile::new(TileData::TONG(TileNumber::THREE)),
        Tile::new(TileData::TONG(TileNumber::THREE)),
        Tile::new(TileData::TONG(TileNumber::THREE)),
        Tile::new(TileData::TONG(TileNumber::THREE)),
        Tile::new(TileData::TONG(TileNumber::FOUR)),
        Tile::new(TileData::TONG(TileNumber::FOUR)),
        Tile::new(TileData::TONG(TileNumber::FOUR)),
        Tile::new(TileData::TONG(TileNumber::FOUR)),
        // Skip one copy of TONG_5
        Tile::new(TileData::TONG(TileNumber::FIVE)),
        Tile::new(TileData::TONG(TileNumber::FIVE)),
        Tile::new(TileData::TONG(TileNumber::FIVE)),
        Tile::new(TileData::TONG(TileNumber::SIX)),
        Tile::new(TileData::TONG(TileNumber::SIX)),
        Tile::new(TileData::TONG(TileNumber::SIX)),
        Tile::new(TileData::TONG(TileNumber::SIX)),
        Tile::new(TileData::TONG(TileNumber::SEVEN)),
        Tile::new(TileData::TONG(TileNumber::SEVEN)),
        Tile::new(TileData::TONG(TileNumber::SEVEN)),
        Tile::new(TileData::TONG(TileNumber::SEVEN)),
        Tile::new(TileData::TONG(TileNumber::EIGHT)),
        Tile::new(TileData::TONG(TileNumber::EIGHT)),
        Tile::new(TileData::TONG(TileNumber::EIGHT)),
        Tile::new(TileData::TONG(TileNumber::EIGHT)),
        Tile::new(TileData::TONG(TileNumber::NINE)),
        Tile::new(TileData::TONG(TileNumber::NINE)),
        Tile::new(TileData::TONG(TileNumber::NINE)),
        Tile::new(TileData::TONG(TileNumber::NINE)),
        Tile::new(TileData::BAMBOO(TileNumber::ONE)),
        Tile::new(TileData::BAMBOO(TileNumber::ONE)),
        Tile::new(TileData::BAMBOO(TileNumber::ONE)),
        Tile::new(TileData::BAMBOO(TileNumber::ONE)),
        Tile::new(TileData::BAMBOO(TileNumber::TWO)),
        Tile::new(TileData::BAMBOO(TileNumber::TWO)),
        Tile::new(TileData::BAMBOO(TileNumber::TWO)),
        Tile::new(TileData::BAMBOO(TileNumber::TWO)),
        Tile::new(TileData::BAMBOO(TileNumber::THREE)),
        Tile::new(TileData::BAMBOO(TileNumber::THREE)),
        Tile::new(TileData::BAMBOO(TileNumber::THREE)),
        Tile::new(TileData::BAMBOO(TileNumber::THREE)),
        Tile::new(TileData::BAMBOO(TileNumber::FOUR)),
        Tile::new(TileData::BAMBOO(TileNumber::FOUR)),
        Tile::new(TileData::BAMBOO(TileNumber::FOUR)),
        Tile::new(TileData::BAMBOO(TileNumber::FOUR)),
        // Skip one copy of BAMBOO_5
        Tile::new(TileData::BAMBOO(TileNumber::FIVE)),
        Tile::new(TileData::BAMBOO(TileNumber::FIVE)),
        Tile::new(TileData::BAMBOO(TileNumber::FIVE)),
        Tile::new(TileData::BAMBOO(TileNumber::SIX)),
        Tile::new(TileData::BAMBOO(TileNumber::SIX)),
        Tile::new(TileData::BAMBOO(TileNumber::SIX)),
        Tile::new(TileData::BAMBOO(TileNumber::SIX)),
        Tile::new(TileData::BAMBOO(TileNumber::SEVEN)),
        Tile::new(TileData::BAMBOO(TileNumber::SEVEN)),
        Tile::new(TileData::BAMBOO(TileNumber::SEVEN)),
        Tile::new(TileData::BAMBOO(TileNumber::SEVEN)),
        Tile::new(TileData::BAMBOO(TileNumber::EIGHT)),
        Tile::new(TileData::BAMBOO(TileNumber::EIGHT)),
        Tile::new(TileData::BAMBOO(TileNumber::EIGHT)),
        Tile::new(TileData::BAMBOO(TileNumber::EIGHT)),
        Tile::new(TileData::BAMBOO(TileNumber::NINE)),
        Tile::new(TileData::BAMBOO(TileNumber::NINE)),
        Tile::new(TileData::BAMBOO(TileNumber::NINE)),
        Tile::new(TileData::BAMBOO(TileNumber::NINE)),
        Tile::new(TileData::WIND(Wind::EAST)),
        Tile::new(TileData::WIND(Wind::EAST)),
        Tile::new(TileData::WIND(Wind::EAST)),
        Tile::new(TileData::WIND(Wind::EAST)),
        Tile::new(TileData::WIND(Wind::SOUTH)),
        Tile::new(TileData::WIND(Wind::SOUTH)),
        Tile::new(TileData::WIND(Wind::SOUTH)),
        Tile::new(TileData::WIND(Wind::SOUTH)),
        Tile::new(TileData::WIND(Wind::WEST)),
        Tile::new(TileData::WIND(Wind::WEST)),
        Tile::new(TileData::WIND(Wind::WEST)),
        Tile::new(TileData::WIND(Wind::WEST)),
        Tile::new(TileData::WIND(Wind::NORTH)),
        Tile::new(TileData::WIND(Wind::NORTH)),
        Tile::new(TileData::WIND(Wind::NORTH)),
        Tile::new(TileData::WIND(Wind::NORTH)),
        Tile::new(TileData::DRAGON(Dragon::WHITE)),
        Tile::new(TileData::DRAGON(Dragon::WHITE)),
        Tile::new(TileData::DRAGON(Dragon::WHITE)),
        Tile::new(TileData::DRAGON(Dragon::WHITE)),
        Tile::new(TileData::DRAGON(Dragon::GREEN)),
        Tile::new(TileData::DRAGON(Dragon::GREEN)),
        Tile::new(TileData::DRAGON(Dragon::GREEN)),
        Tile::new(TileData::DRAGON(Dragon::GREEN)),
        Tile::new(TileData::DRAGON(Dragon::RED)),
        Tile::new(TileData::DRAGON(Dragon::RED)),
        Tile::new(TileData::DRAGON(Dragon::RED)),
        Tile::new(TileData::DRAGON(Dragon::RED)),
    ]);

    // Add missing fives
    let mut special_fives = vec![
        Tile::new(TileData::MAN(TileNumber::FIVE)),
        Tile::new(TileData::TONG(TileNumber::FIVE)),
        Tile::new(TileData::BAMBOO(TileNumber::FIVE)),
    ];

    if flags.has_akadora {
        special_fives
            .iter_mut()
            .for_each(|tile| tile.is_akadora = true);
    }

    base_deck.extend(special_fives);

    // Add flowers, seasons
    if flags.has_flowers {
        base_deck.push_back(Tile::new(TileData::FLOWER(Flower::PLUM)));
        base_deck.push_back(Tile::new(TileData::FLOWER(Flower::LILY)));
        base_deck.push_back(Tile::new(TileData::FLOWER(Flower::CHRYSANTHEMUM)));
        base_deck.push_back(Tile::new(TileData::FLOWER(Flower::BAMBOO)));
    }
    if flags.has_seasons {
        base_deck.push_back(Tile::new(TileData::SEASON(Season::SPRING)));
        base_deck.push_back(Tile::new(TileData::SEASON(Season::SUMMER)));
        base_deck.push_back(Tile::new(TileData::SEASON(Season::AUTUMN)));
        base_deck.push_back(Tile::new(TileData::SEASON(Season::WINTER)));
    }

    // TODO: shuffle
    base_deck
}
