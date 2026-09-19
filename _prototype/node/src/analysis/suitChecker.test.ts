import { Suit } from '../constants/tiles';
import { parseHandString } from '../utils/hand';
import { SuitChecker } from './suitChecker';

describe('suitChecker', () => {
  describe('11123455678999m', () => {
    const tiles = parseHandString('11123455678999m');
    it.skip('parses', () => {
      const suitChecker = SuitChecker.from(Suit.MAN, tiles);

      const branches = suitChecker.parseBranches();
      console.log(
        branches.length,
        branches.map((p) => p.toString())
      );
    });
  });

  describe('11123344556m', () => {
    const tiles = parseHandString('11123344556m');
    it.skip('parses', () => {
      const suitChecker = SuitChecker.from(Suit.MAN, tiles);

      const branches = suitChecker.parseBranches();
      console.log(
        branches.length,
        branches.map((p) => p.toString())
      );
    });
  });

  describe('456m356678p3s2477z', () => {
    const tiles = parseHandString('456m356678p3s2477z');
    it.skip('checks MAN', () => {
      const suitChecker = SuitChecker.from(Suit.MAN, tiles);
      const branches = suitChecker.findBest();

      console.log('leaves', '' + branches);
    });

    it.skip('checks BAMBOO', () => {
      const suitChecker = SuitChecker.from(Suit.BAMBOO, tiles);
      const branches = suitChecker.findBest();

      console.log('leaves', '' + branches);
    });

    it.skip('checks TONG', () => {
      const suitChecker = SuitChecker.from(Suit.TONG, tiles);
      const branches = suitChecker.findBest();

      console.log('leaves', '' + branches);
    });

    it.skip('checks HONOR', () => {
      const suitChecker = SuitChecker.from('HONOR', tiles);
      const branches = suitChecker.parseHonors();

      console.log('leaves', '' + branches);
    });
  });
});
