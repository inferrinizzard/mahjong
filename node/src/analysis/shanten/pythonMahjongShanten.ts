import { ParseBranch } from "../branch";
import { ComboTotal } from "./types";

export const pythonMahjongShanten = (
  total: ComboTotal,
  numTiles: number,
  combo?: ParseBranch[]
) => {
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

  return retShanten;
};
