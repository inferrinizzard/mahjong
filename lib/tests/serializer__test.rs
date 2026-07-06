use mahjong_lib::{
    Tile,
    notation::{Serializer, structs::TileString},
};

#[test]
fn test_serialize_hand() {
    let tiles = vec![Tile::MAN_1, Tile::MAN_2, Tile::MAN_3];

    let tile_string = Serializer::serialize_tiles(tiles);

    assert_eq!(tile_string, TileString::from("123m"))
}

#[test]
fn test_serialize_segmented() {
    let tiles = vec![Tile::MAN_1, Tile::MAN_2, Tile::MAN_3];

    let tile_string = Serializer::serialize_tiles_full(tiles);

    assert_eq!(tile_string, TileString::from("1m2m3m"))
}

#[test]
fn test_serialize_all() {
    let tiles = Tile::TILE_LIST.to_vec();

    let tile_string = Serializer::serialize_tiles(tiles);

    assert_eq!(
        tile_string,
        TileString::from("123456789m123456789p123456789s1234567z12345678f1j")
    )
}
