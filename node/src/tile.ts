import {
  UnicodeTileLookup,
  type Suit,
  type Honor,
  type Season,
  type Flower,
  type Wind,
} from "./constants/tiles";
import { type ValueOf } from "./types/util";

export type TileString = keyof typeof UnicodeTileLookup;

export type TileValue =
  | 1
  | 2
  | 3
  | 4
  | 5
  | 6
  | 7
  | 8
  | 9
  | keyof typeof Wind
  | keyof typeof Honor
  | keyof typeof Season
  | keyof typeof Flower;

export class Tile {
  suit: keyof typeof Suit;
  value: TileValue;
  name: TileString;
  unicode: ValueOf<typeof UnicodeTileLookup>;

  isNumber: boolean;

  constructor(suit: keyof typeof Suit, value: TileValue) {
    this.suit = suit;
    this.value = value;

    this.name = `${value}_${suit}` as TileString;
    this.unicode = UnicodeTileLookup[this.name];
  }

  // static next(tile: Tile, step: number = 1) {
  //   let next_value = tile.value;

  //   if (typeof next_value === "number") {
  //     next_value = ((next_value + step) % 9) as TileValue;
  //   }
  // }
}
