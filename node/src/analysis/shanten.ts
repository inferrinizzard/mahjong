import { Suit } from "../constants/tiles";
import { type TileMap } from "../types/tile";
import { ParseBranch } from "./branch";
import { SuitChecker } from "./suitChecker";

export class Shanten {
  tiles: TileMap;
  num_tiles: number;
  comboes: ParseBranch[][] = [];

  wildcards: number = 0;

  shanten: number = 8;

  constructor(tiles: TileMap) {
    this.tiles = tiles;
    this.num_tiles = Object.values(tiles).reduce(
      (sum, n) => sum + n,
      0 as number
    );
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

    this.calculateShanten();
    return this.shanten;
  }

  calculateShanten = () => {
    // maximumShanten = min(8 - 2 * groups - max(pairs + taatsu, floor(hand.length/3)-groups) - min(1, max(0, pairs + taatsu - (4 - groups))), 6)
    let shantens: number[] = []; // add ukeire as obj prop
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

      const jidahai = combo[3].quads.length - +(this.num_tiles % 3 === 2);

      let curShanten = 8 - total.sets * 2 - total.tatsu - total.pairs;
      let possibleSets = total.sets + total.tatsu;

      if (total.pairs) {
        possibleSets += total.pairs - 1;
      }

      if (possibleSets > 4) {
        curShanten += possibleSets - 4;
      }

      if (curShanten != -1 && curShanten < jidahai) {
        curShanten = jidahai;
      }

      shantens.push(curShanten);
    });

    const minShanten = shantens.reduce((min, num) => (num < min ? num : min));
    this.shanten = minShanten;
  };
}
