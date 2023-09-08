import { type TileNumber } from "../types/tile";

export type Kernel =
  | readonly [0 | 1, 0 | 1, 0 | 1]
  | readonly [0 | 1, 0 | 1, 0 | 1, 0 | 1];

export const matchesKernel = (
  arr: readonly number[],
  start: keyof typeof arr & number,
  kernel: Kernel
) => {
  let total = 0;

  if (start + kernel.length > arr.length) {
    return 0;
  }

  for (let i = 0; i < kernel.length; i++) {
    total += arr[start + i] * kernel[i];
  }

  // return # of copies
  return Math.floor(total / kernel.length);
};

export const pairAt = (i: number): [number, number] => [i, i];
export const tripleAt = (i: number) => [i, i, i];
export const quadAt = (i: number) => [i, i, i, i];
export const straightFrom = (i: number) => [i, i + 1, i + 2];
export const adjTatsu = (i: number): [number, number] => [i, i + 1];
export const skipTatsu = (i: number): [number, number] => [i, i + 2];
