# mahjong

# TODO:

- add unit test for shanten
- validate with logs from tenhou
- use to check if win / agari
- add ukeire number / improvement tiles
- STRETCH: add scoring function

POST:

- add support for wildcards
- add support for check declared sets and discard
- add support for akadora
  - update string parser, icons, move off unicode

- starting building game logic and state in rust
- refactor into 34array format
- add points calculator
  - yaku style
  - mcr style

# arch

## sections

- game client
  - game repr <-> idl transformer
  - bot game ai
  - game server
- solver lib
  - holds base structs also used by game client
  - shanten solver
  - ukeire identifier
  - point scorer
  -
- web client
  - web repr <-> idl transformer
- idl
  - algebraic notation
  -
