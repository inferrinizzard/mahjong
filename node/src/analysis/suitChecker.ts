import { Count } from "../types/tile";
import { ParseBranch, BranchItemMap, BranchItem } from "./branch";
import {
  matchesKernel,
  tripleAt,
  straightFrom,
  adjTatsu,
  quadAt,
  pairAt,
} from "./utils";

const straightKernel = [1, 1, 1] as const;
const adjTatsuKernel = [1, 1, 0] as const; //[0,1,1]
const skipTatsuKernel = [1, 0, 1] as const;
const straightLookaheadKernel = [1, 0, 1, 1] as const;

export class SuitChecker {
  branches: ParseBranch[];
  leaves: ParseBranch[];

  curBranch: ParseBranch;

  i = 0;

  constructor(tiles: Count[]) {
    this.curBranch = new ParseBranch(tiles);

    this.branches = [this.curBranch];
    this.leaves = [];
  }

  splitBranch = (items: BranchItemMap) => {
    const newBranch = new ParseBranch(this.curBranch.hand);

    for (const key in items) {
      for (const item of items[key]) {
        if (key === "singles") {
          newBranch.addSingle(item);
        } else {
          newBranch.addItem(key as Exclude<BranchItem, "singles">, item);
        }
      }
    }

    if (newBranch.hand.length) {
      this.branches.push(newBranch);
    } else {
      this.leaves.push(newBranch);
    }
  };

  check() {
    return true; // STUB
  }
}
