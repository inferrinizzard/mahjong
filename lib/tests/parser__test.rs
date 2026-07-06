use mahjong_lib::{
    Tile,
    notation::{Parser, structs::TileString},
};

#[test]
fn test_parse_hand() {
    let tile_string = TileString::from("123m");

    let result = Parser::parse_tile_string(tile_string);

    assert!(result.is_ok());

    if let Ok(tiles) = result {
        assert_eq!(tiles, vec![Tile::MAN_1, Tile::MAN_2, Tile::MAN_3])
    }
}

#[test]
fn test_parse_segmented() {
    let tile_string = TileString::from("1m2m3m");

    let result = Parser::parse_tile_string(tile_string);

    assert!(result.is_ok());

    if let Ok(tiles) = result {
        assert_eq!(tiles, vec![Tile::MAN_1, Tile::MAN_2, Tile::MAN_3])
    }
}

#[test]
fn test_parse_repeated() {
    let tile_string = TileString::from("1m12m3s4p1m2333m5z1m");

    let result = Parser::parse_tile_string(tile_string);

    assert!(result.is_ok());

    if let Ok(tiles) = result {
        assert_eq!(
            tiles,
            vec![
                Tile::MAN_1,
                Tile::MAN_1,
                Tile::MAN_2,
                Tile::BAMBOO_3,
                Tile::TONG_4,
                Tile::MAN_1,
                Tile::MAN_2,
                Tile::MAN_3,
                Tile::MAN_3,
                Tile::MAN_3,
                Tile::WHITE_DRAGON,
                Tile::MAN_1,
            ]
        )
    }
}
