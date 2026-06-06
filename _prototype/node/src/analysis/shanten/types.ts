import { ParseBranch } from "../branch";

export interface ShantenResult {
  value: number;
  score: number;
  style: "standard" | "7pairs" | "13orphans";
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
) => ShantenResult;
