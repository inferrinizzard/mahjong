import { Suit } from "../constants/tiles";
import { parseHandString } from "../utils/hand";
import { SuitChecker } from "./suitChecker";

// const suitChecker = new SuitChecker([4, 3, 2, 1, 0]);
// console.log("start", "" + suitChecker.branches);
// const branches = suitChecker.parseBranches();

// console.log("leaves", "" + branches);

const tiles = parseHandString("456m356678p3s2477z");

const suitChecker = SuitChecker.from(Suit.MAN, tiles);
console.log("start", "" + suitChecker.branches);
const branches = suitChecker.parseBranches();

console.log("leaves", "" + branches);
