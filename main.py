from sys import stdin
from typing import List, Tuple

from mahjong_types import Deck
from src.deck import build_deck, shuffle_deck
from src.hand import GlobalWind, Hand


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

    while True:
        line = stdin.readline().strip()

        if line == 'done':
            break


if __name__ == '__main__':
    main()
