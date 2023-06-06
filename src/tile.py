from enum import Enum


class Suit(Enum):
    BAMBOO = 'BAMBOO'
    MAN = 'MAN'
    TONG = 'TONG'
    HONOR = 'HONOR'
    WIND = 'WIND'
    FLOWER = 'FLOWER'
    SEASON = 'SEASON'


class Wind(Enum):
    EAST = 1
    SOUTH = 2
    WEST = 3
    NORTH = 4


class Honor(Enum):
    RED = 1
    GREEN = 2
    WHITE = 3


class Season(Enum):
    SPRING = 'EAST'
    SUMMER = 'SOUTH'
    AUTUMN = 'WEST'
    WINTER = 'NORTH'


class Flower(Enum):
    PLUM = 'EAST'
    LILY = 'SOUTH'
    CHRYSANTHEMUM = 'WEST'
    BAMBOO = 'NORTH'


class Tile():
    suit: Suit
    value: int

    def __init__(self, suit: Suit, value: int):
        self.suit = suit
        self.value = value

    def __str__(self) -> str:
        return '{value}_{suit}'.format(value=self.value, suit=self.suit.value)

    def __repr__(self) -> str:
        return '{value} OF {suit}'.format(value=self.value, suit=self.suit)
