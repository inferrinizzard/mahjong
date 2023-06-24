from typing import List
import unittest

from src.hand import GlobalWind, Hand
from src.tile import Suit, Tile
from src.win import is_win


def create_test_hand(nums: List[int], suit=Suit.MAN):
    temp_hand = Hand(name='TEMP', wind=GlobalWind.EAST)
    temp_deck = [Tile(suit, num) for num in nums]
    for _ in range(0, len(temp_deck)):
        temp_hand.draw(temp_deck)

    return temp_hand


def with_suit(nums: List[int], suit=Suit.MAN):
    return tuple([f'{i}_{suit.name}' for i in nums])


class TestWinCheck(unittest.TestCase):

    def test_base_case(self):
        test_hand = create_test_hand([1, 2, 3])
        (_, res) = is_win(test_hand)
        self.assertEqual(res['combos'], [with_suit([1, 2, 3])])

    def test_double_sequence(self):
        test_hand = create_test_hand([1, 2, 3, 4, 5, 6])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([4, 5, 6])]
        )

    def test_overlapping_sequence(self):
        test_hand = create_test_hand([1, 2, 3, 3, 4, 5])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([3, 4, 5])]
        )

    def test_sequence_with_pair(self):
        test_hand = create_test_hand([1, 1, 2, 3, 4])
        (_, res) = is_win(test_hand)
        self.assertEqual(res['combos'], [with_suit([2, 3, 4])])
        self.assertEqual(res['pairs'], [with_suit([1, 1])])

    def test_sequence_cluster(self):
        test_hand = create_test_hand([1, 2, 2, 3, 3, 4, 4, 5, 6])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([2, 3, 4]), with_suit([4, 5, 6])]
        )

    def test_double_sequence_with_orphan(self):
        test_hand = create_test_hand([1, 1, 2, 2, 3, 3, 4])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([1, 2, 3])]
        )
        self.assertEqual(res['orphans'], [f'{4}_MAN'])

    def test_sequence_with_ambiguous_triple(self):
        test_hand = create_test_hand([1, 1, 1, 2, 2, 3, 3, 4])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([2, 3, 4])]
        )
        self.assertEqual(res['pairs'], [with_suit([1, 1])])

    def test_sequence_with_middle_pair(self):
        test_hand = create_test_hand([1, 2, 3, 4, 4, 5, 6, 7])
        (_, res) = is_win(test_hand)
        self.assertEqual(
            res['combos'],
            [with_suit([1, 2, 3]), with_suit([5, 6, 7])]
        )
        self.assertEqual(res['pairs'], [with_suit([4, 4])])
