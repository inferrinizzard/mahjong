from random import shuffle

from mahjong_types import Deck
from . import tile


def build_deck(include_bonus=False) -> Deck:
    tile_types = {k.lower(): v for k, v in tile.__dict__.items()
                  if not k.startswith('_')}

    # print(Suit._member_map_)

    deck = []
    for suit in tile.Suit:
        suit_name = suit.name.lower()
        suit_values = tile_types.get(suit_name, [i for i in range(1, 10)])

        copies = 4

        if suit == tile.Suit.SEASON or suit == tile.Suit.FLOWER:
            if not include_bonus:
                continue
            copies = 1

        for value in suit_values:
            for _ in range(0, copies):
                deck.append(tile.Tile(suit=suit, value=value))

    return deck


def shuffle_deck(deck: Deck) -> Deck:
    shuffle(deck)
    return deck
