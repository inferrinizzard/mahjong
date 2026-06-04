import { UnicodeTileLookup } from "../constants/tiles";
import { type TileMap } from "../types/tile";

const suitLookup = {
  m: "MAN",
  p: "TONG",
  s: "BAMBOO",
};

export const honorList = [
  "",
  "EAST_WIND",
  "SOUTH_WIND",
  "WEST_WIND",
  "NORTH_WIND",
  "WHITE_DRAGON",
  "GREEN_DRAGON",
  "RED_DRAGON",
];

export const EmptyHand = Object.fromEntries(
  Object.keys(UnicodeTileLookup).map((key) => [key, 0])
) as TileMap;

export const parseHandString = (hand: string): TileMap => {
  let tileMap: Partial<TileMap> = {};

  const man = hand.matchAll(/\d+m/g);
  const pin = hand.matchAll(/\d+p/g);
  const sou = hand.matchAll(/\d+s/g);
  const honor = hand.matchAll(/\d+z/g);

  for (const suit of [man, pin, sou]) {
    for (const match of suit) {
      const suitString =
        suitLookup[match[0].split("").pop()! as keyof typeof suitLookup];
      for (const char of match[0].replace(/\w$/, "").split("")) {
        const tileString = `${char}_${suitString}` as keyof TileMap;
        if (tileString in tileMap) {
          tileMap[tileString]!++;
        } else {
          tileMap[tileString] = 1;
        }
      }
    }
  }

  for (const match of honor) {
    for (const char of match[0].replace(/\w$/, "").split("")) {
      const tileString = honorList[+char] as keyof TileMap;
      if (tileString in tileMap) {
        tileMap[tileString]!++;
      } else {
        tileMap[tileString] = 1;
      }
    }
  }

  return { ...EmptyHand, ...tileMap };
};
