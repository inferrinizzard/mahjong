import re
from typing import Dict, List

from mahjong_types import Tileset
from src.tile import Tile
from src.util import take


class TileSlice():
    tiles: Dict[str, int]
    suit: str

    def __init__(self, tiles, suit) -> None:
        self.tiles = tiles
        self.suit = suit


def is_win(hand):
    combos = []
    pairs = []
    orphans = []

    # for each numeric suit:
    # find longest chain, determine number of possible ambiguities
    # find highest scoring (most pairs / triples along with sequences)
    numeric_tiles: Tileset = {tile: count for tile,
                              count in hand.tiles.items() if re.search(r'\d', tile)}
    suits = set([key.split('_')[1] for key in numeric_tiles.keys()])
    for suit in suits:
        hand_slice = TileSlice(
            tiles={tile: count for tile, count in numeric_tiles.items() if suit in tile}, suit=suit)

        (sequences, _) = find_sequences(hand_slice)
        # remove sequences from hand
        for sequence in sequences:
            combos.append(tuple(sequence))
            for tile in sequence:
                take(hand.tiles, tile)

        for tile, count in hand.tiles.items():
            if count == 1:
                orphans.append(tile)
            if count == 2:
                pairs.append((tile, tile))
            if count == 3:
                combos.append(('TRIPLE', tile))
            if count == 4:
                combos.append(('QUAD', tile))

    # for others:
    # find triples / pairs
    char_tiles: Tileset = {tile: count for tile,
                           count in hand.tiles.items() if not re.search(r'\d', tile)}
    for tile, count in char_tiles.items():
        if count == 3:
            combos.append(('TRIPLE', tile))
        elif count == 4:
            combos.append(('QUAD', tile))

    return (
        len(orphans) == 0 and len(combos) == 4 and len(pairs) == 1,
        {'combos':  combos, 'pairs': pairs, 'orphans': orphans}
    )


def find_sequences(hand: TileSlice, start=1):
    base_points = calculate_hand_points(hand.tiles)

    best = base_points
    best_sequences = []

    for i in range(start, 10):
        num = num_sequences(hand, i) + 1

        for j in range(0, num):
            copy = hand.tiles.copy()
            sequences = take_sequence(copy, f'{i}_{hand.suit}', j)

            if j == 0:
                continue

            (cur_sequences, cur_points) = find_sequences(
                TileSlice(tiles=copy, suit=hand.suit), i+1)
            points = (3 if len(sequences) == 1 else 4 *
                      len(sequences)) + cur_points

            if points > best:
                best = points
                best_sequences = [*sequences, *cur_sequences]

    return (best_sequences, best)


def num_sequences(hand: TileSlice, index: int):
    first = f'{index}_{hand.suit}'
    second = f'{index + 1}_{hand.suit}'
    third = f'{index + 2}_{hand.suit}'

    if first in hand.tiles and second in hand.tiles and third in hand.tiles:
        return min(hand.tiles[first], hand.tiles[second], hand.tiles[third])
    return 0


def calculate_hand_points(hand: Tileset) -> int:
    points = 0
    for count in hand.values():
        value = -2 if count == 1 else count
        points += value
    return points


def take_sequence(tiles: Tileset, start: str, times=1):
    if times == 0:
        return ([], None)
    sequences: List[List[str]] = []

    first = start
    second = Tile.next_from_str(start)
    third = Tile.next_from_str(second)
    sequence = [first, second, third]

    count = min(tiles[first], tiles[second], tiles[third], times)
    for _ in range(0, count):
        for tile in sequence:
            take(tiles, tile)
        sequences.append(sequence)

    return sequences
