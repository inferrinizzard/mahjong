#[derive(Debug)]
pub enum Suit {
    MAN,
    TONG,
    BAMBOO,
    WIND,
    DRAGON,
    FLOWER,
    SEASON,
}

#[derive(Debug)]
pub enum Wind {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

#[derive(Debug)]
pub enum Dragon {
    WHITE,
    GREEN,
    RED,
}

#[derive(Debug)]
pub enum Flower {
    PLUM,
    LILY,
    CHRYSANTHEMUM,
    BAMBOO,
}

#[derive(Debug)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}
