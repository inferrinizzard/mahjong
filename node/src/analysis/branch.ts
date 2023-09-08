import { type Count } from "../types/tile";

export type BranchItem = ("sets" | "pairs" | "tatsu" | "singles") &
  keyof ParseBranch;

export type BranchItemMap<ItemType extends BranchItem = BranchItem> =
  ItemType extends ItemType ? Record<ItemType, ParseBranch[ItemType]> : never;

export class ParseBranch {
  hand: Count[];
  sets: number[][] = [];
  pairs: [number, number][] = [];
  tatsu: [number, number][] = [];
  singles: number[] = [];

  constructor(hand: Count[]) {
    this.hand = hand;
  }

  addItem = <ItemType extends Exclude<BranchItem, "singles">>(
    itemType: ItemType,
    item: this[ItemType][number]
  ) => {
    for (let i = 0; i < item.length; i++) {
      this.hand[item[i]]--;
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
