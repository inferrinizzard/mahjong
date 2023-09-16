import { UnicodeTileLookup, type Wind } from "./constants/tiles";
import { type BitMap, type TileMap } from "./types/tile";

export class Hand {
  wind: keyof typeof Wind;
  tileMap: TileMap;
  tileBitMap: BitMap;
  name: string;

  constructor(wind: keyof typeof Wind, name: string = "player") {
    this.name = name;
    this.wind = wind;

    this.tileMap = Object.fromEntries(
      Object.keys(UnicodeTileLookup).map((key) => [key, 0])
    ) as TileMap;
    this.tileBitMap = Object.values(this.tileMap) as BitMap;
  }
}
