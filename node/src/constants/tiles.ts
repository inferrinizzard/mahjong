export const Suit = Object.freeze({
  BAMBOO: "BAMBOO",
  MAN: "MAN",
  TONG: "TONG",
  HONOR: "HONOR",
  WIND: "WIND",
  FLOWER: "FLOWER",
  SEASON: "SEASON",
} as const);

export const Wind = Object.freeze({
  EAST: "EAST",
  SOUTH: "SOUTH",
  WEST: "WEST",
  NORTH: "NORTH",
} as const);

export const Honor = Object.freeze({
  RED: "RED",
  GREEN: "GREEN",
  WHITE: "WHITE",
} as const);

export const Season = Object.freeze({
  SPRING: "EAST",
  SUMMER: "SOUTH",
  AUTUMN: "WEST",
  WINTER: "NORTH",
} as const);

export const Flower = Object.freeze({
  PLUM: "EAST",
  LILY: "SOUTH",
  CHRYSANTHEMUM: "WEST",
  BAMBOO: "NORTH",
} as const);

export const UnicodeTileLookup = Object.freeze({
  "1_BAMBOO": "\u{0001F010}",
  "2_BAMBOO": "\u{0001F011}",
  "3_BAMBOO": "\u{0001F012}",
  "4_BAMBOO": "\u{0001F013}",
  "5_BAMBOO": "\u{0001F014}",
  "6_BAMBOO": "\u{0001F015}",
  "7_BAMBOO": "\u{0001F016}",
  "8_BAMBOO": "\u{0001F017}",
  "9_BAMBOO": "\u{0001F018}",
  "1_MAN": "\u{0001F007}",
  "2_MAN": "\u{0001F008}",
  "3_MAN": "\u{0001F009}",
  "4_MAN": "\u{0001F00A}",
  "5_MAN": "\u{0001F00B}",
  "6_MAN": "\u{0001F00C}",
  "7_MAN": "\u{0001F00D}",
  "8_MAN": "\u{0001F00E}",
  "9_MAN": "\u{0001F00F}",
  "1_TONG": "\u{0001F019}",
  "2_TONG": "\u{0001F01A}",
  "3_TONG": "\u{0001F01B}",
  "4_TONG": "\u{0001F01C}",
  "5_TONG": "\u{0001F01D}",
  "6_TONG": "\u{0001F01E}",
  "7_TONG": "\u{0001F01F}",
  "8_TONG": "\u{0001F020}",
  "9_TONG": "\u{0001F021}",
  RED_HONOR: "\u{0001F004}",
  GREEN_HONOR: "\u{0001F005}",
  WHITE_HONOR: "\u{0001F006}",
  EAST_WIND: "\u{0001F000}",
  SOUTH_WIND: "\u{0001F001}",
  WEST_WIND: "\u{0001F002}",
  NORTH_WIND: "\u{0001F003}",
  PLUM_FLOWER: "\u{0001F022}",
  LILY_FLOWER: "\u{0001F023}",
  CHRYSANTHEMUM_FLOWER: "\u{0001F024}",
  BAMBOO_FLOWER: "\u{0001F025}",
  SPRING_SEASON: "\u{0001F026}",
  SUMMER_SEASON: "\u{0001F027}",
  AUTUMN_SEASON: "\u{0001F028}",
  WINTER_SEASON: "\u{0001F029}",
} as const);
