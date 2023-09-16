export type Kernel =
  | readonly [0 | 1, 0 | 1, 0 | 1]
  | readonly [0 | 1, 0 | 1, 0 | 1, 0 | 1];

export const matchesKernel = (
  arr: readonly number[],
  start: keyof typeof arr & number,
  kernel: Kernel
) => {
  let res = new Array(kernel.length).fill(0);

  if (start + kernel.length > arr.length) {
    return 0;
  }

  for (let i = 0; i < kernel.length; i++) {
    const matches = !kernel[i] || !!kernel[i] === !!arr[start + i];
    if (!matches) {
      return 0;
    }

    res[i] = arr[start + i];
  }

  // return # of copies
  return res.reduce((min, n) => (n ? Math.min(min, n) : min));
};

export const pairAt = (i: number): [number, number] => [i, i];
export const tripleAt = (i: number) => [i, i, i];
export const quadAt = (i: number) => [i, i, i, i];
export const straightFrom = (i: number) => [i, i + 1, i + 2];
export const adjTatsu = (i: number): [number, number] => [i, i + 1];
export const skipTatsu = (i: number): [number, number] => [i, i + 2];
