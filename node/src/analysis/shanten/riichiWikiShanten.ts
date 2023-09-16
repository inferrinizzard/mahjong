import { ComboTotal } from "./types";

export const riichiWikiShanten = (total: ComboTotal, numTiles: number) => {
  // maximumShanten = min(8 - 2 * groups - max(pairs + taatsu, floor(hand.length/3)-groups) - min(1, max(0, pairs + taatsu - (4 - groups))), 6)
  const groups = total.sets;
  const preGroups = total.pairs + total.tatsu;

  const curShanten =
    8 -
    2 * groups -
    Math.max(preGroups, Math.floor(numTiles / 3) - groups) -
    Math.min(1, Math.max(0, preGroups - (4 - groups)));

  return curShanten;
};
