import { Count } from "../types/tile";
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

  splitBranch = (items: BranchItemMap) => {
    const newBranch = new ParseBranch(
      this.curBranch!.hand.slice(0),
      this.curBranch!.sets.slice(0),
      this.curBranch!.pairs.slice(0),
      this.curBranch!.tatsu.slice(0),
      this.curBranch!.singles.slice(0)
    );

    for (const key in items) {
      for (const item of items[key]) {
        if (key === "singles") {
          newBranch.addSingle(item);
        } else {
          newBranch.addItem(key as Exclude<BranchItem, "singles">, item);
        }
      }
    }

    if (newBranch.isEmpty()) {
      this.leaves.push(newBranch);
    } else {
      this.branches.push(newBranch);
    }
  };

  check() {
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
  }
}
