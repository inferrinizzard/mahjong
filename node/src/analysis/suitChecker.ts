import { Count, NumericSuit, TileMap } from "../types/tile";
import { honorList } from "../utils/hand";
import { ParseBranch, BranchItemMap, BranchItem } from "./branch";
import {
  matchesKernel,
  tripleAt,
  straightFrom,
  adjTatsu,
  quadAt,
  pairAt,
  skipTatsu,
} from "./utils";

const straightKernel = [1, 1, 1] as const;
const adjTatsuKernel = [1, 1, 0] as const; //[0,1,1]
const skipTatsuKernel = [1, 0, 1] as const;

export class SuitChecker {
  branches: ParseBranch[];
  leaves: ParseBranch[];

  curBranch: ParseBranch | null;

  constructor(tiles: Count[]) {
    this.curBranch = null;

    this.branches = [new ParseBranch(tiles)];
    this.leaves = [];
  }

  static from = (suit: NumericSuit | "HONOR", tileMap: TileMap) => {
    let tiles;
    if (suit === "HONOR") {
      tiles = new Array(7)
        .fill(0)
        .map((_, i) => tileMap[honorList[i + 1] as keyof TileMap]);
    } else {
      tiles = new Array(9)
        .fill(0)
        .map((_, i) => tileMap[`${i + 1}_${suit}` as keyof TileMap]);
    }

    return new SuitChecker(tiles);
  };

  splitBranch = (items: BranchItemMap) => {
    const newBranch = new ParseBranch(
      this.curBranch!.hand.slice(0),
      this.curBranch!.sets.slice(0),
      this.curBranch!.pairs.slice(0),
      this.curBranch!.tatsu.slice(0),
      this.curBranch!.singles.slice(0)
    );

    for (const key in items) {
      // @ts-expect-error
      for (const item of items[key]) {
        newBranch.addItem(key as BranchItem, item);
      }
    }

    if (newBranch.isEmpty()) {
      this.leaves.push(newBranch);
    } else {
      this.branches.push(newBranch);
    }
  };

  parseBranches() {
    while (this.branches.length) {
      this.curBranch = this.branches.shift()!;
      const start = this.curBranch.hand.findIndex((n) => n);
      // console.log("current:", start, "" + this.curBranch);

      //
      if (this.curBranch.hand[start] === 4) {
        // AAA, ABC
        if (matchesKernel(this.curBranch.hand, start, straightKernel)) {
          this.splitBranch({
            sets: [tripleAt(start), straightFrom(start)],
          });
        }

        // AAA, AB_
        if (matchesKernel(this.curBranch.hand, start, adjTatsuKernel)) {
          this.splitBranch({
            sets: [tripleAt(start)],
            tatsu: [adjTatsu(start)],
          });
        }

        // AAA, A_C
        if (matchesKernel(this.curBranch.hand, start, skipTatsuKernel)) {
          this.splitBranch({
            sets: [tripleAt(start)],
            tatsu: [skipTatsu(start)],
          });
        }

        // AAAA
        {
          this.splitBranch({ sets: [quadAt(start)] });
        }
      }
      //
      else if (this.curBranch.hand[start] === 3) {
        // AA, ABC
        if (matchesKernel(this.curBranch.hand, start, straightKernel)) {
          this.splitBranch({
            pairs: [pairAt(start)],
            sets: [straightFrom(start)],
          });
        }

        // AA, AB_
        if (matchesKernel(this.curBranch.hand, start, adjTatsuKernel)) {
          this.splitBranch({
            tatsu: [adjTatsu(start)],
            pairs: [pairAt(start)],
          });
        }

        // AA, A_C
        if (matchesKernel(this.curBranch.hand, start, skipTatsuKernel)) {
          this.splitBranch({
            tatsu: [skipTatsu(start)],
            pairs: [pairAt(start)],
          });
        }

        // AAA
        {
          this.splitBranch({ sets: [tripleAt(start)] });
        }
      }
      //
      else if (this.curBranch.hand[start] === 2) {
        // ABC, A
        if (matchesKernel(this.curBranch.hand, start, straightKernel)) {
          this.splitBranch({ sets: [straightFrom(start)], singles: [start] });
        }

        // A, AB_
        if (matchesKernel(this.curBranch.hand, start, adjTatsuKernel)) {
          this.splitBranch({ tatsu: [adjTatsu(start)], singles: [start] });
        }

        // A, A_C
        if (matchesKernel(this.curBranch.hand, start, skipTatsuKernel)) {
          this.splitBranch({ tatsu: [skipTatsu(start)], singles: [start] });
        }

        // AA
        {
          this.splitBranch({ pairs: [pairAt(start)] });
        }
      }
      //
      else if (this.curBranch.hand[start] === 1) {
        // ABC
        if (matchesKernel(this.curBranch.hand, start, straightKernel)) {
          this.splitBranch({ sets: [straightFrom(start)] });
        }

        // AB_
        if (matchesKernel(this.curBranch.hand, start, adjTatsuKernel)) {
          this.splitBranch({ tatsu: [adjTatsu(start)] });
        }

        // A_C
        if (matchesKernel(this.curBranch.hand, start, skipTatsuKernel)) {
          this.splitBranch({ tatsu: [skipTatsu(start)] });
        }

        // A
        {
          this.splitBranch({ singles: [start] });
        }
      }
    }

    this.sortBranches();
    return this.leaves;
  }

  sortBranches() {
    this.leaves = this.leaves.sort((a, b) => {
      const aScore = a.score || a.calculateScore();
      const bScore = b.score || b.calculateScore();

      return (
        bScore - aScore ||
        b.sets.length - a.sets.length ||
        b.pairs.length - a.pairs.length ||
        b.tatsu.length - a.tatsu.length ||
        a.singles.length - b.singles.length
      );
    });
  }

  findBest = () => {
    this.parseBranches();

    const best = this.leaves.reduce(
      (best, leaf) =>
        leaf.score! > best.score
          ? { score: leaf.score!, branches: [leaf] }
          : leaf.score === best.score
          ? { score: best.score, branches: [...best.branches, leaf] }
          : best,
      { score: 0, branches: [] as ParseBranch[] }
    );

    return best.branches;
  };

  parseHonors = () => {
    const branch = this.branches.pop()!;

    for (let i = 0; i < branch.hand.length; i++) {
      if (branch.hand[i] === 4) {
        branch.addSet(quadAt(i));
      } else if (branch.hand[i] === 3) {
        branch.addSet(tripleAt(i));
      } else if (branch.hand[i] === 2) {
        branch.addSet(pairAt(i));
      } else {
        branch.addSingle(i);
      }
    }

    return branch;
  };
}
