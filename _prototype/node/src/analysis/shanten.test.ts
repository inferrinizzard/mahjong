import { parseHandString } from "../utils/hand";
import { Shanten } from "./shanten";
import { pythonMahjongShanten, riichiWikiShanten } from "./shanten/solver";

describe("shanten", () => {
  // describe("456m356678p3s2477z", () => {
  //   const tiles = parseHandString("456m356678p3s2477z");
  //   const shanten = new Shanten(tiles);

  //   it("finds the correct shanten", () => {
  //     expect(shanten.find()).toBe(2);
  //   });

  //   it.todo("finds the correct ukeire", () => {});
  // });

  describe.only("7777m1p789s44447z8m", () => {
    const tiles = parseHandString("7777m1p789s44447z8m");

    it("finds the correct shanten with pythonMahjong", () => {
      const shanten = new Shanten(tiles, pythonMahjongShanten);
      expect(shanten.find().value).toBe(1);
    });

    it("finds the correct shanten with riichiWiki", () => {
      const shanten = new Shanten(tiles, riichiWikiShanten);
      expect(shanten.find().value).toBe(1);
    });

    it.todo("finds the correct ukeire", () => {});
  });
});
