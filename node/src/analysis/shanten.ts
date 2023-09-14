import { Dragon, Suit, Wind } from "../constants/tiles";
import {
  type TileMap,
  type TileNumber,
  type Count,
  type NumericSuit,
} from "../types/tile";
import { ParseBranch } from "./branch";
import { SuitChecker } from "./suitChecker";

export class Shanten {
  tiles: TileMap;
  comboes: ParseBranch[][] = [];

  wildcards: number = 0;

  shanten: number = 8;

  constructor(tiles: TileMap) {
    this.tiles = tiles;
  }

  find() {
    const { man, tong, bamboo, honor } = this.splitIntoSuits(this.tiles);

    const manBranches = SuitChecker.from(Suit.MAN, this.tiles).findBest();
    const tongBranches = SuitChecker.from(Suit.MAN, this.tiles).findBest();
    const bambooBranches = SuitChecker.from(Suit.MAN, this.tiles).findBest();
    const honorBranch = this.parseHonors(honor);

    manBranches.forEach((man) =>
      tongBranches.forEach((tong) =>
        bambooBranches.forEach((bamboo) =>
          this.comboes.push([man, tong, bamboo, honorBranch])
        )
      )
    );
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
    return suitChecker.findBest();
  }

  parseHonors = <
    HonorSlice extends Pick<
      TileMap,
      | `${keyof typeof Wind}_${typeof Suit.WIND}`
      | `${keyof typeof Dragon}_${typeof Suit.DRAGON}`
    >
  >(
    slice: HonorSlice
  ) => {
    const tileArr = Object.values(slice);

    const suitChecker = new SuitChecker(tileArr);
    return suitChecker.parseHonors();
  };

  calculateShanten = () => {
    let minShanten = 8; // max
    this.comboes.forEach((combo) => {
      const total = combo.reduce(
        (sum, branch) => ({
          sets: sum.sets + branch.sets.length,
          pairs: sum.pairs + branch.pairs.length,
          tatsu: sum.tatsu + branch.tatsu.length,
          singles: sum.singles + branch.singles.length,
        }),
        { sets: 0, pairs: 0, tatsu: 0, singles: 0 }
      );

      const numHonors = combo[3].numTiles;

      let curShanten = 8 - total.sets * 2 - total.tatsu - total.pairs;
      let possibleSets = total.sets + total.tatsu;

      if (total.pairs) {
        possibleSets += total.pairs - 1;
      }
      //   elif self.number_characters and self.number_isolated_tiles:
      //       if (self.number_characters | self.number_isolated_tiles) == self.number_characters:
      //           ret_shanten += 1

      if (possibleSets > 4) {
        curShanten += possibleSets - 4;
      }

      if (curShanten != -1 && curShanten < numHonors) {
        curShanten = numHonors;
      }

      if (curShanten < minShanten) {
        minShanten = curShanten;
      }
    });

    this.shanten = minShanten;
  };
}
