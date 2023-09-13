import { type Count } from "../types/tile";

export type BranchItem = ("sets" | "pairs" | "tatsu" | "singles") &
  keyof ParseBranch;

export type BranchItemMap<ItemType extends BranchItem = BranchItem> =
  ItemType extends ItemType ? Record<ItemType, ParseBranch[ItemType]> : never;

export class ParseBranch {
  numTiles: number = 0;
  hand: Count[]; // use Uint8Array
  sets: number[][] = [];
  pairs: [number, number][] = [];
  tatsu: [number, number][] = [];
  singles: number[] = [];
  score: number | null = null;

  constructor(
    hand: Count[],
    sets?: number[][],
    pairs?: [number, number][],
    tatsu?: [number, number][],
    singles?: number[]
  ) {
    this.hand = hand;
    this.numTiles = hand.reduce((sum, count) => sum + count, 0 as number);

    if (sets) {
      this.sets = sets;
    }
    if (pairs) {
      this.pairs = pairs;
    }
    if (tatsu) {
      this.tatsu = tatsu;
    }
    if (singles) {
      this.singles = singles;
    }
  }

  toString() {
    return `
	score: ${this.score}
	hand: [ ${this.hand.join(", ")} ]
	sets: [ ${this.sets.map((item) => `(${item.join(",")})`).join(" ")} ] 
	pairs: [ ${this.pairs.map((item) => `(${item.join(",")})`).join(" ")} ] 
	tatsu: [ ${this.tatsu.map((item) => `(${item.join(",")})`).join(" ")} ] 
	singles: [ ${this.singles.join(", ")} ]
		`;
  }

  isEmpty = () => {
    if (this.hand.some((count) => count < 0)) {
      console.trace();
      throw new Error(this.toString());
    }
    return this.hand.length === 0 || this.hand.every((count) => count === 0);
  };

  calculateScore = () => {
    let sum = 0;
    sum += this.sets.length * 3;
    sum += this.pairs.length * 2;
    sum += this.tatsu.length * 1;
    sum -= this.singles.length * 1;

    this.score = sum;
    return sum;
  };

  addItem = <ItemType extends BranchItem>(
    itemType: ItemType,
    item: this[ItemType][number]
  ) => {
    if (itemType === "singles") {
      this.addSingle(item as number);
      return;
    }

    for (let i of item as number[]) {
      this.hand[i]--;
    }

    this[itemType].push(item as any);

    return this.hand;
  };

  addSingle = (item: number) => {
    this.hand[item]--;
    this.singles.push(item);
    return this.hand;
  };

  addPair = <N extends number>(pair: [N, N]) => {
    this.addItem("pairs", pair);
  };

  addSet = (set: number[]) => {
    this.addItem("sets", set);
  };

  addTatsu = (tatsu: [number, number]) => {
    this.addItem("tatsu", tatsu);
  };
}
