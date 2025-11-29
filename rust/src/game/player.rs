use derivative::Derivative;

use crate::{consts::Wind, structs::Hand};

#[derive(Derivative)]
#[derivative(Default)]
pub struct Player {
    pub name: String,
    pub hand: Hand,

    #[derivative(Default(value = "Wind::EAST"))]
    pub wind: Wind,

    #[derivative(Default(value = "0"))]
    pub points: i8,
    #[derivative(Default(value = "false"))]
    pub is_dealer: bool,
    #[derivative(Default(value = "0"))]
    pub win_streak: usize,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            ..Default::default()
        }
    }
}
