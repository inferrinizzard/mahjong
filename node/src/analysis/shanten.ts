import { Dragon, Suit, Wind } from "../constants/tiles";
import {
  type TileMap,
  type TileNumber,
  type Count,
  type NumericSuit,
} from "../types/tile";
import { SuitChecker } from "./suitChecker";

export class Shanten {
  tiles: TileMap;

  // suitChecker: SuitChecker;

  wildcards: number = 0;

  constructor(tiles: TileMap) {
    this.tiles = tiles;
  }

  find() {
    const { man, tong, bamboo, honor } = this.splitIntoSuits(this.tiles);

    this.checkSuit(man);
  }

  splitIntoSuits(tilemap: TileMap) {
    const manTiles = Object.fromEntries(
      Object.entries(tilemap).filter(([key]) => key.includes(Suit.MAN))
    ) as Pick<TileMap, `${TileNumber}_${typeof Suit.MAN}`>;
    const tongTiles = Object.fromEntries(
      Object.entries(tilemap).filter(([key]) => key.includes(Suit.TONG))
    ) as Pick<TileMap, `${TileNumber}_${typeof Suit.TONG}`>;
    const bambooTiles = Object.fromEntries(
      Object.entries(tilemap).filter(([key]) => key.includes(Suit.BAMBOO))
    ) as Pick<TileMap, `${TileNumber}_${typeof Suit.BAMBOO}`>;
    const honorTiles = Object.fromEntries(
      Object.entries(tilemap).filter(
        ([key]) => key.includes(Suit.WIND) || key.includes(Suit.DRAGON)
      )
    ) as Pick<
      TileMap,
      | `${keyof typeof Wind}_${typeof Suit.WIND}`
      | `${keyof typeof Dragon}_${typeof Suit.DRAGON}`
    >;

    return {
      man: manTiles,
      tong: tongTiles,
      bamboo: bambooTiles,
      honor: honorTiles,
    };
  }

  checkSuit<
    Suit extends NumericSuit,
    Slice extends Suit extends Suit
      ? Record<`${TileNumber}_${Suit}`, Count>
      : never
  >(slice: Slice) {
    const tileArr = Object.values(slice);

    const suitChecker = new SuitChecker(tileArr);
    return suitChecker.check();
  }
}
