from sys import stdin
from typing import List, Tuple

from mahjong_types import Deck
from src.deck import build_deck, shuffle_deck
from src.hand import GlobalWind, Hand
from src.tile import Suit, Tile
from src.win import is_win


def deal_hands(deck: Deck) -> Tuple[Deck, List[Hand]]:
    hands = []
    for i in range(0, 4):
        hand = Hand(name=chr(ord('a') + i), wind=GlobalWind(i))
        for _ in range(0, 13):
            hand.draw(deck)
        hands.append(hand)

    return (deck, hands)


def main():
    deck = build_deck()
    deck = shuffle_deck(deck)
    (deck, hands) = deal_hands(deck)
    print([str(hand) for hand in hands])

    turn = 0
    mod_turn = 0

    discard = [[], [], [], []]

    while True:
        line = stdin.readline().strip()

        if line == 'deal':
            mod_turn = turn % 4

            hand = hands[mod_turn]

            tile = hand.draw(deck)

            print(hand.__repr__(), tile)

            hand.play(tile)
            discard[mod_turn].append(tile)

            turn += 1

        if line == 'done':
            break


if __name__ == '__main__':
    temp_hand = Hand(name='TEMP', wind=GlobalWind.EAST)
    temp_deck = [
        Tile(Suit.MAN, 1),
        Tile(Suit.MAN, 1),
        Tile(Suit.MAN, 1),
        Tile(Suit.MAN, 1),
        Tile(Suit.MAN, 2),
        Tile(Suit.MAN, 2),
        Tile(Suit.MAN, 3),
        Tile(Suit.MAN, 3),
        Tile(Suit.MAN, 4),
    ]
    for _ in range(0, len(temp_deck)):
        temp_hand.draw(temp_deck)

    is_win(temp_hand)
    # main()
