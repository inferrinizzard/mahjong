from enum import Enum


class Suit(Enum):
    BAMBOO = 'BAMBOO'
    MAN = 'MAN'
    TONG = 'TONG'
    HONOR = 'HONOR'
    WIND = 'WIND'
    FLOWER = 'FLOWER'
    BIRD = 'BIRD'


class Wind(Enum):
    EAST = 1
    SOUTH = 2
    WEST = 3
    NORTH = 4


class Honor(Enum):
    RED = 1
    GREEN = 2
    WHITE = 3


class Tile():
    suit: Suit
    value: int

    def __init__(self, suit: Suit, value: int):
        self.suit = suit
        self.value = value

    def __repr__(self) -> str:
        return '{value} OF {suit}'.format(value=self.value, suit=self.suit)
