import {
  type UnicodeTileLookup,
  type Wind,
  type Dragon,
  type Season,
  type Flower,
  type Suit,
} from "../constants/tiles";

export type NumericSuit =
  | typeof Suit.MAN
  | typeof Suit.TONG
  | typeof Suit.BAMBOO;

export type TileString = keyof typeof UnicodeTileLookup;

export type TileNumber = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

export type TileValue =
  | TileNumber
  | keyof typeof Wind
  | keyof typeof Dragon
  | keyof typeof Season
  | keyof typeof Flower;

export type TileMap = Record<TileString, Count>;

// TODO: refine
export type TileSet =
  | {
      style: "TRIPLE";
      tiles: [TileString, TileString, TileString];
    }
  | {
      style: "QUAD";
      tiles: [TileString, TileString, TileString, TileString];
    }
  | {
      style: "STRAIGHT";
      tiles: [TileString, TileString, TileString];
    };

export type Count = 0 | 1 | 2 | 3 | 4;

export type BitMap = [
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count,
  Count
];
