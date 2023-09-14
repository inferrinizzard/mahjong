import { Suit } from "../constants/tiles";
import { type TileMap } from "../types/tile";
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
    const manBranches = SuitChecker.from(Suit.MAN, this.tiles).findBest();
    const tongBranches = SuitChecker.from(Suit.TONG, this.tiles).findBest();
    const bambooBranches = SuitChecker.from(Suit.BAMBOO, this.tiles).findBest();
    const honorBranch = SuitChecker.from("HONOR", this.tiles).parseHonors();

    manBranches.forEach((man) =>
      tongBranches.forEach((tong) =>
        bambooBranches.forEach((bamboo) =>
          this.comboes.push([man, tong, bamboo, honorBranch])
        )
      )
    );
  }

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
