from . import tile


def build_deck(include_bonus=False):
    tile_types = {k.lower(): v for k, v in tile.__dict__.items()
                  if not k.startswith('_')}

    deck = []
    for suit in tile.Suit:
        if not include_bonus:
            if suit == tile.Suit.BIRD or suit == tile.Suit.FLOWER:
                continue
        suit_name = suit.name.lower()
        suit_values = tile_types.get(suit_name, [i for i in range(1, 10)])

        for value in suit_values:
            for _ in range(0, 4):
                deck.append(tile.Tile(suit=suit, value=value))

    print(deck, len(deck))