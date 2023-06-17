from enum import Enum
from typing import Dict

from mahjong_types import Deck


class GlobalWind(Enum):
    EAST = 0
    SOUTH = 1
    WEST = 2
    NORTH = 3


class Hand():
    wind: GlobalWind
    tiles: Dict[str, int]
    name: str

    def __init__(self, wind: GlobalWind, name: str = 'PLAYER'):
        self.tiles = {}
        self.wind = wind
        self.name = name

    def __str__(self):
        return 'name: {name}   wind: {wind}'.format(name=self.name, wind=self.wind)

    def __repr__(self) -> str:
        return self.__str__() + ' ' + str(self.tiles)

    def draw(self, deck: Deck) -> str:
        tile = str(deck.pop())
        self.tiles[tile] = self.tiles.get(tile, 0) + 1
        return tile

    def play(self, tile: str):
        self.tiles[tile] -= 1
        if self.tiles[tile] == 0:
            del self.tiles[tile]
