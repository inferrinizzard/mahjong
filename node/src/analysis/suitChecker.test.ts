import { Suit } from "../constants/tiles";
import { parseHandString } from "../utils/hand";
import { SuitChecker } from "./suitChecker";

describe("suitChecker", () => {
  describe("456m356678p3s2477z", () => {
    const tiles = parseHandString("456m356678p3s2477z");
    it("checks MAN", () => {
      const suitChecker = SuitChecker.from(Suit.MAN, tiles);
      const branches = suitChecker.findBest();

      console.log("leaves", "" + branches);
    });

    it("checks BAMBOO", () => {
      const suitChecker = SuitChecker.from(Suit.BAMBOO, tiles);
      const branches = suitChecker.findBest();

      console.log("leaves", "" + branches);
    });

    it("checks TONG", () => {
      const suitChecker = SuitChecker.from(Suit.TONG, tiles);
      const branches = suitChecker.findBest();

      console.log("leaves", "" + branches);
    });
  });
});
