export type Suit = keyof typeof Suit;
export const Suit = Object.freeze({
  BAMBOO: 'BAMBOO',
  MAN: 'MAN',
  TONG: 'TONG',
  DRAGON: 'DRAGON',
  WIND: 'WIND',
  FLOWER: 'FLOWER',
  SEASON: 'SEASON',
} as const);

export type Wind = keyof typeof Wind;
export const Wind = Object.freeze({
  EAST: 'EAST',
  SOUTH: 'SOUTH',
  WEST: 'WEST',
  NORTH: 'NORTH',
} as const);

export type Dragon = keyof typeof Dragon;
export const Dragon = Object.freeze({
  WHITE: 'WHITE',
  GREEN: 'GREEN',
  RED: 'RED',
} as const);

export type Season = keyof typeof Season;
export const Season = Object.freeze({
  SPRING: 'EAST',
  SUMMER: 'SOUTH',
  AUTUMN: 'WEST',
  WINTER: 'NORTH',
} as const);

export type Flower = keyof typeof Flower;
export const Flower = Object.freeze({
  PLUM: 'EAST',
  LILY: 'SOUTH',
  CHRYSANTHEMUM: 'WEST',
  BAMBOO: 'NORTH',
} as const);

// missing akadora, include joker tile ?
export const UnicodeTileLookup = Object.freeze({
  '1_BAMBOO': '\u{0001F010}',
  '2_BAMBOO': '\u{0001F011}',
  '3_BAMBOO': '\u{0001F012}',
  '4_BAMBOO': '\u{0001F013}',
  '5_BAMBOO': '\u{0001F014}',
  '6_BAMBOO': '\u{0001F015}',
  '7_BAMBOO': '\u{0001F016}',
  '8_BAMBOO': '\u{0001F017}',
  '9_BAMBOO': '\u{0001F018}',
  '1_MAN': '\u{0001F007}',
  '2_MAN': '\u{0001F008}',
  '3_MAN': '\u{0001F009}',
  '4_MAN': '\u{0001F00A}',
  '5_MAN': '\u{0001F00B}',
  '6_MAN': '\u{0001F00C}',
  '7_MAN': '\u{0001F00D}',
  '8_MAN': '\u{0001F00E}',
  '9_MAN': '\u{0001F00F}',
  '1_TONG': '\u{0001F019}',
  '2_TONG': '\u{0001F01A}',
  '3_TONG': '\u{0001F01B}',
  '4_TONG': '\u{0001F01C}',
  '5_TONG': '\u{0001F01D}',
  '6_TONG': '\u{0001F01E}',
  '7_TONG': '\u{0001F01F}',
  '8_TONG': '\u{0001F020}',
  '9_TONG': '\u{0001F021}',
  WHITE_DRAGON: '\u{0001F006}',
  GREEN_DRAGON: '\u{0001F005}',
  RED_DRAGON: '\u{0001F004}',
  EAST_WIND: '\u{0001F000}',
  SOUTH_WIND: '\u{0001F001}',
  WEST_WIND: '\u{0001F002}',
  NORTH_WIND: '\u{0001F003}',
  PLUM_FLOWER: '\u{0001F022}',
  LILY_FLOWER: '\u{0001F023}',
  CHRYSANTHEMUM_FLOWER: '\u{0001F024}',
  BAMBOO_FLOWER: '\u{0001F025}',
  SPRING_SEASON: '\u{0001F026}',
  SUMMER_SEASON: '\u{0001F027}',
  AUTUMN_SEASON: '\u{0001F028}',
  WINTER_SEASON: '\u{0001F029}',
} as const);

export const nextTileMap = Object.freeze({
  '1_BAMBOO': '2_BAMBOO',
  '2_BAMBOO': '3_BAMBOO',
  '3_BAMBOO': '4_BAMBOO',
  '4_BAMBOO': '5_BAMBOO',
  '5_BAMBOO': '6_BAMBOO',
  '6_BAMBOO': '7_BAMBOO',
  '7_BAMBOO': '8_BAMBOO',
  '8_BAMBOO': '9_BAMBOO',
  '9_BAMBOO': '1_BAMBOO',
  '1_MAN': '2_MAN',
  '2_MAN': '3_MAN',
  '3_MAN': '4_MAN',
  '4_MAN': '5_MAN',
  '5_MAN': '6_MAN',
  '6_MAN': '7_MAN',
  '7_MAN': '8_MAN',
  '8_MAN': '9_MAN',
  '9_MAN': '1_MAN',
  '1_TONG': '2_TONG',
  '2_TONG': '3_TONG',
  '3_TONG': '4_TONG',
  '4_TONG': '5_TONG',
  '5_TONG': '6_TONG',
  '6_TONG': '7_TONG',
  '7_TONG': '8_TONG',
  '8_TONG': '9_TONG',
  '9_TONG': '1_TONG',
  WHITE_DRAGON: 'GREEN_DRAGON',
  GREEN_DRAGON: 'RED_DRAGON',
  RED_DRAGON: 'WHITE_DRAGON',
  EAST_WIND: 'SOUTH_WIND',
  SOUTH_WIND: 'WEST_WIND',
  WEST_WIND: 'NORTH_WIND',
  NORTH_WIND: 'EAST_WIND',
  PLUM_FLOWER: 'LILY_FLOWER',
  LILY_FLOWER: 'CHRYSANTHEMUM_FLOWER',
  CHRYSANTHEMUM_FLOWER: 'BAMBOO_FLOWER',
  BAMBOO_FLOWER: 'PLUM_FLOWER',
  SPRING_SEASON: 'SUMMER_SEASON',
  SUMMER_SEASON: 'AUTUMN_SEASON',
  AUTUMN_SEASON: 'WINTER_SEASON',
  WINTER_SEASON: 'SPRING_SEASON',
} as const);

export const suitCodeMap = Object.freeze({
  BAMBOO: 's',
  MAN: 'm',
  TONG: 'p',
  DRAGON: 'z',
  WIND: 'z',
  FLOWER: 'f',
  SEASON: 'f',
} as const);

export const valueCodeMap = Object.freeze({
  EAST_WIND: 1,
  SOUTH_WIND: 2,
  WEST_WIND: 3,
  NORTH_WIND: 4,
  WHITE_DRAGON: 5,
  GREEN_DRAGON: 6,
  RED_DRAGON: 7,
  PLUM_FLOWER: 1,
  LILY_FLOWER: 2,
  CHRYSANTHEMUM_FLOWER: 3,
  BAMBOO_FLOWER: 4,
  SPRING_SEASON: 5,
  SUMMER_SEASON: 6,
  AUTUMN_SEASON: 7,
  WINTER_SEASON: 8,
} as const);
