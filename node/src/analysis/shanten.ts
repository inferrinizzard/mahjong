import { Suit } from "../constants/tiles";
import { type TileMap } from "../types/tile";
import { ParseBranch } from "./branch";
import { ShantenSolver } from "./shanten/types";
import { SuitChecker } from "./suitChecker";

export class Shanten {
  tiles: TileMap;
  numTiles: number;
  comboes: ParseBranch[][] = [];

  wildcards: number = 0;

  shantenSolver: ShantenSolver;
  shanten: number = 8;

  constructor(tiles: TileMap, shantenSolver: ShantenSolver) {
    this.tiles = tiles;
    this.numTiles = Object.values(tiles).reduce(
      (sum, n) => sum + n,
      0 as number
    );

    this.shantenSolver = shantenSolver;
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

      const curShanten = this.shantenSolver(total, this.numTiles, combo);
      shantens.push(curShanten);
    });

    console.log(shantens);
    const minShanten = shantens.reduce((min, num) => (num < min ? num : min));
    this.shanten = minShanten;
  };
}
