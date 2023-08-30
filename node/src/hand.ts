import { UnicodeTileLookup, type Wind } from "./constants/tiles";
import { type TileString } from "./tile";
import { type Count, type BitMap } from "./types";

export class Hand {
  wind: keyof typeof Wind;
  tileMap: Record<TileString, Count>;
  tileBitMap: BitMap;
  name: string;

  constructor(wind: keyof typeof Wind, name: string = "player") {
    this.name = name;
    this.wind = wind;

    this.tileMap = Object.fromEntries(
      Object.keys(UnicodeTileLookup).map((key) => [key, 0])
    ) as typeof this.tileMap;
    this.tileBitMap = Object.values(this.tileMap) as BitMap;
  }
}
