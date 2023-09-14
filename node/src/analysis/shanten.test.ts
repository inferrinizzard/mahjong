import { parseHandString } from "../utils/hand";
import { Shanten } from "./shanten";

describe("shanten", () => {
  describe("456m356678p3s2477z", () => {
    const tiles = parseHandString("456m356678p3s2477z");
    const shanten = new Shanten(tiles);

    it("finds the correct shanten", () => {
      expect(shanten.find()).toBe(2);
    });

    it.todo("finds the correct ukeire", () => {});
  });
});
