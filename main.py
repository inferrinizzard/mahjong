from typing import List, Tuple
from mahjong_types import Deck, Hand
from src.deck import build_deck, shuffle_deck


def deal_hands(deck: Deck) -> Tuple[Deck, List[Hand]]:
    hands = []
    for _ in range(0, 4):
        hand = {}
        for _ in range(0, 13):
            tile = str(deck.pop())
            hand[tile] = hand.get(tile, 0) + 1
        hands.append(hand)

    return (deck, hands)


def main():
    deck = build_deck()
    deck = shuffle_deck(deck)
    (deck, hands) = deal_hands(deck)


if __name__ == '__main__':
    main()
