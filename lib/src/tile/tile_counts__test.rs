use crate::tile::tile_counts::*;

#[test]
fn tile_counts_from_dense_tile_string() {
    let tile_string = TileString::from("123456789m");

    let result = TileCounts::from(tile_string);

    let mut expected = [0; 34];
    expected[..9].copy_from_slice(&[1; 9]);
    assert_eq!(result, TileCounts { value: expected });
}

#[test]
fn tile_counts_from_segmented_tile_string() {
    let tile_string = TileString::from("1m2m3m4m5m6m7m8m9m");

    let result = TileCounts::from(tile_string);

    let mut expected = [0; 34];
    expected[..9].copy_from_slice(&[1; 9]);
    assert_eq!(result, TileCounts { value: expected });
}
