use mahjong_lib::{notation::structs::TileString, solver::solve_shanten, tile::TileCounts};

const STANDARD_TEST_CASES: [&str; 20] = [
    // A
    "258m258p258s1235z",
    "238m2589p58s1225z",
    "2389m56p1289s235z",
    "3389m56p1289s235z",
    "333m56p1289s2557z",
    "333m567p1289s255z",
    "333m567p12789s55z",
    "",
    // B
    "259m159p258s1235z",
    "259m159p158s1235z",
    "259m159p18s12335z",
    "159m159p18s12335z",
    "199m159p19s12335z",
    "199m199p55s22335z",
    "199m99p55s223355z",
    "",
    // C
    "123m258p258s1235z",
    "111m258p258s1235z",
    "",
    // D
    "458m158p158s12357z",
];

#[test]
fn test_standard_shanten_descending() {
    let starting_shanten = 6;
    let mut active_shanten = starting_shanten;

    for test_case in STANDARD_TEST_CASES {
        if test_case.len() == 0 {
            active_shanten = starting_shanten;
            continue;
        }

        let tile_counts = TileCounts::from(TileString::from(test_case));
        let result = solve_shanten(&tile_counts);

        println!("{}", test_case);

        assert_eq!(active_shanten, result);

        active_shanten -= 1;
    }
}
