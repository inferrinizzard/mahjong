import { UnicodeTileLookup, type Wind } from '../constants/tiles';
import { type BitMap, type TileMap } from '../types/tile';
import { Tile } from './tile';

export class TileHand {
  wind: keyof typeof Wind;
  tiles: Tile[];
  tileMap: TileMap;
  tileBitMap: BitMap;
  name: string;

  constructor(wind: keyof typeof Wind, name: string = 'player') {
    this.name = name;
    this.wind = wind;
    this.tiles = [];

    this.tileMap = Object.fromEntries(
      Object.keys(UnicodeTileLookup).map((key) => [key, 0])
    ) as TileMap;
    this.tileBitMap = Object.values(this.tileMap) as BitMap;
  }

  add_tile(tile: Tile) {
    this.tiles.push(tile);
    this.tileMap[tile.name] += 1;
  }

  sort() {
    this.tiles = this.tiles.sort((a, b) => Tile.less_than(a, b));
  }

  to_code() {
    this.sort();
    const tileCodes = this.tiles.map((tile) => tile.to_code());

    // dedupe ?
    return tileCodes.join('');
  }

  public static from_code() {}
}
