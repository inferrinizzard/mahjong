import { ParseBranch } from "../branch";
import { ComboTotal, ShantenResult } from "./types";

export const pythonMahjongShanten = (
  total: ComboTotal,
  numTiles: number,
  combo?: ParseBranch[]
): ShantenResult => {
  const jidahai = combo![3].quads.length - +(numTiles % 3 === 2);

  let retShanten = 8 - total.sets * 2 - total.tatsu - total.pairs;
  let mentsuKouho = total.sets + total.tatsu;

  if (total.pairs) {
    mentsuKouho += total.pairs - 1;
  }
  //   elif self.number_characters and self.number_isolated_tiles:
  //       if (self.number_characters | self.number_isolated_tiles) == self.number_characters:
  //           ret_shanten += 1

  if (mentsuKouho > 4) {
    retShanten += mentsuKouho - 4;
  }

  if (retShanten != -1 && retShanten < jidahai) {
    retShanten = jidahai;
  }

  return {
    value: retShanten,
    score: retShanten,
    style: "standard",
  };
};

export const riichiWikiShanten = (
  total: ComboTotal,
  numTiles: number
): ShantenResult => {
  // maximumShanten = min(8 - 2 * groups - max(pairs + taatsu, floor(hand.length/3)-groups) - min(1, max(0, pairs + taatsu - (4 - groups))), 6)
  const groups = total.sets;
  const preGroups = total.pairs + total.tatsu;

  const curShanten =
    8 -
    2 * groups -
    Math.max(preGroups, Math.floor(numTiles / 3) - groups) -
    Math.min(1, Math.max(0, preGroups - (4 - groups)));

  return {
    value: curShanten,
    score: curShanten,
    style: "standard",
  };
};

export const sevenPairsShanten = (
  total: ComboTotal,
  numTiles: number
): ShantenResult => {
  const curShanten = 6 - total.pairs;
  return {
    value: curShanten,
    score: curShanten,
    style: "7pairs",
  };
};

export const thirteenOrphansShanten = (
  total: ComboTotal,
  numTiles: number,
  combo?: ParseBranch[]
): ShantenResult => {
  let terminalPairs = 0;
  let diffTerminals = 0;

  for (const branch of combo!.slice(0, -1)) {
    const one = branch.hand[0];
    const nine = branch.hand[8];

    if (one) {
      diffTerminals++;
    }
    if (one >= 2) {
      terminalPairs++;
    }
    if (nine) {
      diffTerminals++;
    }
    if (nine >= 2) {
      terminalPairs++;
    }
  }
  const honors = combo![3];
  diffTerminals += honors.hand.reduce(
    (sum, n) => (n ? sum + 1 : sum),
    0 as number
  );
  terminalPairs += honors.sets.length + honors.pairs.length;

  const curShanten = 13 - diffTerminals - Math.max(terminalPairs, 1);

  return {
    value: curShanten,
    score: curShanten,
    style: "13orphans",
  };
};
