import re
from typing import Dict, List, Tuple

from src.tile import Tile
from src.util import take


def is_win(hand):
    combos = []
    pair = None

    # for each numeric suit:
    # find longest chain, determine number of possible ambiguities
    # find highest scoring (most pairs / triples along with sequences)
    numeric_tiles: Dict[str, int] = {tile: count for tile,
                                     count in hand.tiles.items() if re.search(r'\d', tile)}
    suits = set([key.split('_')[1] for key in numeric_tiles.keys()])
    print(suits)
    for suit in suits:
        index = 1
        filtered_hand: Dict[str, int] = {tile: count for tile,
                                         count in numeric_tiles.items() if suit in tile}
        sequences: List[Tuple[str, int]] = []

        while index < 9:
            length = 1
            tile = f'{index}_{suit}'
            if tile in numeric_tiles:
                length = find_longest_sequence(tile, numeric_tiles)
                # print(tile, length)

                if length >= 3:
                    sequences.append((tile, length))
            index += length

        print(sequences)

        res = find_sequences(filtered_hand, sequences, length)
        print(res)

    # for others:
    # find triples / pairs
    char_tiles: Dict[str, int] = {tile: count for tile,
                                  count in hand.tiles.items() if not re.search(r'\d', tile)}
    for tile, count in char_tiles.items():
        if count == 3:
            combos.append(('TRIPLE', tile))
        elif count == 4:
            combos.append(('QUAD', tile))

    pass


def find_longest_sequence(tile: str, hand: Dict[str, int]):
    if tile.split('_')[0] == 9:
        return 1

    length = 1
    next_tile = Tile.next_from_str(tile)
    while next_tile and next_tile in hand:
        length += 1
        next_tile = Tile.next_from_str(next_tile)

    return length


def find_sequences(hand: Dict[str, int], sequences, length: int):
    final_sequences = []
    all_used = False

    for tile, length in sequences:
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

            for tile, count in temp_hand.items():
                if count == 1:
                    points -= 1
                    continue
                temp_sequences.append([tile] * count)
                points += count

            orphans = [tile for tile, count in temp_hand.items() if count == 1]

            print('Remaining hand:', temp_hand)
            print('sequences', temp_sequences, points)

            if points > best:
                best = points
                final_sequences = temp_sequences
                all_used = len(orphans) == 0

    return (final_sequences, all_used)


def take_sequence(hand: Dict[str, int], start: str):
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
