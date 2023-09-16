import { Suit } from "../constants/tiles";
import { type TileMap } from "../types/tile";
import { ParseBranch } from "./branch";
import { sevenPairsShanten, thirteenOrphansShanten } from "./shanten/solver";
import { ShantenResult, ShantenSolver } from "./shanten/types";
import { SuitChecker } from "./suitChecker";

export class Shanten {
  tiles: TileMap;
  numTiles: number;
  comboes: ParseBranch[][] = [];

  wildcards: number = 0;

  shantenSolver: ShantenSolver;
  shantens: ShantenResult[] = [];

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

    const standardShanten = this.findStandardShanten();
    const sevenPairsShanten = this.findSevenPairsShanten();
    const thirteenPairsShanten = this.findThirteenPairsShanten();

    this.shantens = [
      ...standardShanten,
      ...sevenPairsShanten,
      ...thirteenPairsShanten,
    ].sort((a, b) => a.value - b.value || a.score - b.score);

    return this.shantens[0];
  }

  findAll() {
    if (!this.shantens.length) {
      this.find();
    }
    return this.shantens;
  }

  findSevenPairsShanten(): ShantenResult[] {
    let shantens: ShantenResult[] = [];
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

      const curShanten = sevenPairsShanten(total, this.numTiles);
      shantens.push(curShanten);
    });

    // console.log(shantens);
    return shantens;
  }

  findThirteenPairsShanten(): ShantenResult[] {
    let shantens: ShantenResult[] = [];
    this.comboes.forEach((combo) => {
      const curShanten = thirteenOrphansShanten(
        { sets: 0, pairs: 0, tatsu: 0, singles: 0 },
        this.numTiles,
        combo
      );
      shantens.push(curShanten);
    });

    // console.log(shantens);
    return shantens;
  }

  findStandardShanten = () => {
    let shantens: ShantenResult[] = [];
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

    // console.log(shantens);
    return shantens;
  };
}
