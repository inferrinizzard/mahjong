import {
  nextTileMap,
  Suit,
  suitCodeMap,
  UnicodeTileLookup,
  valueCodeMap,
} from './constants/tiles';

import { type TileValue, type TileString } from './types/tile';
import { type ValueOf } from './types/util';

export class Tile {
  suit: Suit;
  value: TileValue;
  name: TileString;
  unicode: ValueOf<typeof UnicodeTileLookup>;

  public isAkadora: boolean = false;
  public isDora: boolean = false;
  public isWild: boolean = false;

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

  public to_code(): string {
    if (this.isWild) {
      return '0j';
    }

    const suitCode = suitCodeMap[this.suit];
    if (this.isNumber) {
      const value = this.isAkadora ? 0 : this.value;
      return `${value}${suitCode}`;
    }

    if (this.isHonor || this.isBonus) {
      const value = valueCodeMap[this.name as keyof typeof valueCodeMap];
      return `${value}${suitCode}`;
    }

    return '';
  }

  public static from_string(name: string): Tile {
    const [value, suit, ..._] = name.split('_');
    return new Tile(suit as Suit, value as TileValue);
  }

  public static next(tile: Tile, step: number = 1): Tile {
    const nextTileName = nextTileMap[tile.name];

    return Tile.from_string(nextTileName);
  }
}
