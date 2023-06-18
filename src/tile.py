from enum import Enum
from typing import Union


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


UNICODE_LOOKUP = {
    '1_BAMBOO': u'\U0001F010',
    '2_BAMBOO': u'\U0001F011',
    '3_BAMBOO': u'\U0001F012',
    '4_BAMBOO': u'\U0001F013',
    '5_BAMBOO': u'\U0001F014',
    '6_BAMBOO': u'\U0001F015',
    '7_BAMBOO': u'\U0001F016',
    '8_BAMBOO': u'\U0001F017',
    '9_BAMBOO': u'\U0001F018',
    '1_MAN': u'\U0001F007',
    '2_MAN': u'\U0001F008',
    '3_MAN': u'\U0001F009',
    '4_MAN': u'\U0001F00A',
    '5_MAN': u'\U0001F00B',
    '6_MAN': u'\U0001F00C',
    '7_MAN': u'\U0001F00D',
    '8_MAN': u'\U0001F00E',
    '9_MAN': u'\U0001F00F',
    '1_TONG': u'\U0001F019',
    '2_TONG': u'\U0001F01A',
    '3_TONG': u'\U0001F01B',
    '4_TONG': u'\U0001F01C',
    '5_TONG': u'\U0001F01D',
    '6_TONG': u'\U0001F01E',
    '7_TONG': u'\U0001F01F',
    '8_TONG': u'\U0001F020',
    '9_TONG': u'\U0001F021',
    'Honor.RED_HONOR': u'\U0001F004',
    'Honor.GREEN_HONOR': u'\U0001F005',
    'Honor.WHITE_HONOR': u'\U0001F006',
    'Wind.EAST_WIND': u'\U0001F000',
    'Wind.SOUTH_WIND': u'\U0001F001',
    'Wind.WEST_WIND': u'\U0001F002',
    'Wind.NORTH_WIND': u'\U0001F003',
    'Flower.PLUM_FLOWER': u'\U0001F022',
    'Flower.LILY_FLOWER': u'\U0001F023',
    'Flower.CHRYSANTHEMUM_FLOWER': u'\U0001F024',
    'Flower.BAMBOO_FLOWER': u'\U0001F025',
    'Season.SPRING_SEASON': u'\U0001F026',
    'Season.SUMMER_SEASON': u'\U0001F027',
    'Season.AUTUMN_SEASON': u'\U0001F028',
    'Season.WINTER_SEASON': u'\U0001F029',
}


class Tile():
    suit: Suit
    value: Union[int, Wind, Honor, Season, Flower]
    string: str
    unicode: str

    # flags
    is_numeric: bool
    # is_terminal: bool
    # is_green: bool
    # is_symmetric: bool

    def __init__(self, suit: Suit, value: Union[int, Wind, Honor, Season, Flower]):
        self.suit = suit
        self.value = value

        self.string = '{value}_{suit}'.format(
            value=self.value, suit=self.suit.value)
        self.unicode = UNICODE_LOOKUP[self.string]

        self.is_numeric = type(self.value) == int

    def __str__(self) -> str:
        return '{unicode}  {string}'.format(unicode=self.unicode, string=self.string)

    def __repr__(self) -> str:
        return self.string

    @staticmethod
    def next(tile):
        next_value = tile.value
        value_type = type(tile.value)

        if value_type == int:
            next_value = (tile.value + 1) % 9
        elif issubclass(value_type, Enum):
            next_value = value_type((tile.value.value + 1) % len(value_type))
        else:  # flower and season
            return None

        return '{value}_{suit}'.format(value=next_value, suit=tile.suit.value)

    @staticmethod
    def next_from_str(tile: str):
        return Tile.next(Tile.from_str(tile))

    @staticmethod
    def from_str(tile: str):
        value, suit = tile.split('_')
        return Tile(suit=Suit(suit), value=int(value) if value.isdigit() else value)
