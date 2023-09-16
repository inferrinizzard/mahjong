from mahjong.hand_calculating.hand import HandCalculator
from mahjong.tile import TilesConverter
from mahjong.hand_calculating.hand_config import HandConfig
from mahjong.meld import Meld

from mahjong.shanten import Shanten

# 456m356678p3s2477z
# tiles = TilesConverter.string_to_34_array(sou='3', man='456', pin='356678', honors='2477')

# 177m14p788s44447z8m
tiles = TilesConverter.string_to_34_array(sou='789', man='77778', pin='1', honors='44447')


shantest = Shanten()
# shantest._init(tiles)

res = shantest.calculate_shanten(tiles)
print(res)

