import { ParseBranch } from "../branch";

export interface Shanten {
  num: number;
  style: "standard" | "7pair" | "13orphans";
  ukeire?: number[];
}

export interface ComboTotal {
  sets: number;
  pairs: number;
  tatsu: number;
  singles: number;
}

export type ShantenSolver = (
  total: ComboTotal,
  numTiles: number,
  combo?: ParseBranch[]
) => number;
