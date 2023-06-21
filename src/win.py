import re
from typing import Dict, List, Tuple

from src.tile import Tile
from src.util import take

Tileset = Dict[str, int]


def is_win(hand):
    combos = []
    pair = None

    # for each numeric suit:
    # find longest chain, determine number of possible ambiguities
    # find highest scoring (most pairs / triples along with sequences)
    numeric_tiles: Tileset = {tile: count for tile,
                              count in hand.tiles.items() if re.search(r'\d', tile)}
    suits = set([key.split('_')[1] for key in numeric_tiles.keys()])
    print(suits)
    for suit in suits:
        filtered_hand: Tileset = {tile: count for tile,
                                  count in numeric_tiles.items() if suit in tile}

        res = find_sequences(filtered_hand, suit)
        print(res)

    # for others:
    # find triples / pairs
    char_tiles: Tileset = {tile: count for tile,
                           count in hand.tiles.items() if not re.search(r'\d', tile)}
    for tile, count in char_tiles.items():
        if count == 3:
            combos.append(('TRIPLE', tile))
        elif count == 4:
            combos.append(('QUAD', tile))

    pass


def find_longest_sequence(tile: str, hand: Tileset):
    if tile.split('_')[0] == 9:
        return 1

    length = 1
    next_tile = Tile.next_from_str(tile)
    while next_tile and next_tile in hand:
        length += 1
        next_tile = Tile.next_from_str(next_tile)

    return length


def find_chains(hand: Tileset, suit: str):
    chains: List[Tuple[str, int]] = []
    index = 1

    print(hand)

    while index < 9:
        length = 1
        tile = f'{index}_{suit}'
        if tile in hand:
            length = find_longest_sequence(tile, hand)
            # print(tile, length)

            if length >= 3:
                chains.append((tile, length))
        index += length

    print('chains:', chains)

    return chains


def find_sequences(hand: Tileset, suit: str):
    chains = find_chains(hand, suit)

    if not chains:
        return ([], calculate_hand_points(hand), len(hand) == 0)

    final_sequences = []
    all_used = False

    for tile, length in chains:
        num = length % 3 + 1
        best = 0

        for i in range(0, num):
            points = 0
            temp_hand = hand.copy()
            temp_sequences = []

            cur_tile = Tile.next_from_str(tile, i)

            num_sequences = (length / 3).__floor__()
            for _ in range(0, num_sequences):
                (cur_sequences, cur_tile) = take_sequence(temp_hand, cur_tile)
                temp_sequences.extend(cur_sequences)
                points += 3 if len(cur_sequences) == 1 else 4 * \
                    len(cur_sequences)

            # calculate points for all existing combos, deducting for orphans
            points += calculate_hand_points(temp_hand)
            for tile, count in temp_hand.items():
                if count > 1:
                    temp_sequences.append([tile] * count)

            # find remaining tiles not used in combos
            orphans = [tile for tile, count in temp_hand.items() if count == 1]

            print('Remaining hand:', temp_hand)
            print('combos', temp_sequences, points)

            if points > best:
                best = points
                final_sequences = temp_sequences
                all_used = len(orphans) == 0

    return (final_sequences, best, all_used)


def calculate_hand_points(hand: Tileset) -> int:
    points = 0
    for count in hand.values():
        value = -1 if count == 1 else count
        points += value
    return points


def take_sequence(hand: Tileset, start: str):
    sequences: List[List[str]] = []

    first = start
    second = Tile.next_from_str(start)
    third = Tile.next_from_str(second)
    sequence = [first, second, third]

    count = min(hand[first], hand[second], hand[third])
    for _ in range(0, count):
        for tile in sequence:
            take(hand, tile)
        sequences.append(sequence)

    next_tile = Tile.next_from_str(third)
    return (sequences, next_tile)
