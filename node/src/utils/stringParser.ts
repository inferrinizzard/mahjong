const suitLookup = {
  m: "MAN",
  p: "TONG",
  s: "BAMBOO",
};

const honorList = [
  "",
  "EAST_WIND",
  "SOUTH_WIND",
  "WEST_WIND",
  "NORTH_WIND",
  "GREEN_DRAGON",
  "WHITE_DRAGON",
  "RED_DRAGON",
];

export const parseHandString = (hand: string) => {
  let tileMap = {};

  const man = hand.matchAll(/\d+m/g);
  const pin = hand.matchAll(/\d+p/g);
  const sou = hand.matchAll(/\d+s/g);
  const honor = hand.matchAll(/\d+z/g);

  for (const suit of [man, pin, sou]) {
    for (const match of suit) {
      const suitString = suitLookup[match[0].split("").pop()!];
      for (const char of match[0].replace(/\w$/, "").split("")) {
        const tileString = `${char}_${suitString}`;
        if (tileString in tileMap) {
          tileMap[tileString]++;
        } else {
          tileMap[tileString] = 1;
        }
      }
    }
  }

  for (const match of honor) {
    for (const char of match[0].replace(/\w$/, "").split("")) {
      const tileString = `${char}_${honorList[+char]}`;
      if (tileString in tileMap) {
        tileMap[tileString]++;
      } else {
        tileMap[tileString] = 1;
      }
    }
  }

  return tileMap;
};
