import { SuitChecker } from "./suitChecker";

const suitChecker = new SuitChecker([4, 3, 2, 1, 0]);
console.log("start", "" + suitChecker.branches);
suitChecker.check();

console.log("leaves", "" + suitChecker.leaves);
