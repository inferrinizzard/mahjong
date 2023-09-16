import { UnicodeTileLookup, type Suit } from "./constants/tiles";
import { type TileValue, type TileString } from "./types/tile";
import { type ValueOf } from "./types/util";

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
