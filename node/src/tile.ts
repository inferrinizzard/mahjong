import { NextTileMap, Suit, UnicodeTileLookup } from './constants/tiles';
import { type TileValue, type TileString } from './types/tile';
import { type ValueOf } from './types/util';

export class Tile {
  suit: Suit;
  value: TileValue;
  name: TileString;
  unicode: ValueOf<typeof UnicodeTileLookup>;

  constructor(suit: keyof typeof Suit, value: TileValue) {
    this.suit = suit;
    this.value = value;

    this.name = `${value}_${suit}` as TileString;
    this.unicode = UnicodeTileLookup[this.name];
  }

  public get isNumber(): boolean {
    return typeof this.value === 'number';
  }
  public get isTerminal(): boolean {
    return (
      typeof this.value === 'number' && (this.value === 1 || this.value === 9)
    );
  }
  public get isSimple(): boolean {
    return typeof this.value === 'number' && (this.value > 1 || this.value < 9);
  }
  public get isBonus(): boolean {
    return this.suit === Suit.SEASON || this.suit === Suit.FLOWER;
  }
  public get isHonor(): boolean {
    return this.suit === Suit.WIND || this.suit === Suit.DRAGON;
  }
  public get isGreen(): boolean {
    return (
      this.value === 'GREEN' ||
      (this.suit === Suit.BAMBOO &&
        (this.value === 2 ||
          this.value === 3 ||
          this.value === 4 ||
          this.value === 6 ||
          this.value === 8))
    );
  }

  public static from_string(name: string): Tile {
    const [value, suit, ..._] = name.split('_');
    return new Tile(suit as Suit, value as TileValue);
  }

  public static next(tile: Tile, step: number = 1): Tile {
    const nextTileName = NextTileMap[tile.name];

    return Tile.from_string(nextTileName);
  }
}
