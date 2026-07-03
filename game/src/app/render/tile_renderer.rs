use std::path::Path;

use iced::{
    Size,
    widget::{container, mouse_area, pin, svg},
};
use mahjong_lib::{consts::Suit, tile::TileData};

use crate::{
    app::{
        Message,
        game::{GameMessage, GameTile},
        render::consts::{Direction, TILE_ASPECT_RATIO, TILE_BACK_PATH, TILE_BACK_X_PATH},
        root::Component,
    },
    util::get_path::get_path,
};

pub struct TileRenderer {
    id: String,
    path: String,
    size: Size,
    direction: Direction,
    position: (f32, f32),
}

// Render methods
impl TileRenderer {
    pub fn render(mut self) -> Component {
        if self.path.is_empty() {
            self.path = get_path(
                if matches!(self.direction, Direction::DOWN | Direction::UP) {
                    TILE_BACK_PATH
                } else {
                    TILE_BACK_X_PATH
                },
            )
        }

        let mut tile_component = self.render_tile_svg();

        if !self.id.is_empty() {
            let message = Message::Game(GameMessage::TileClick(self.id, self.direction));
            tile_component = mouse_area(tile_component).on_press(message).into()
        }

        tile_component = pin(tile_component)
            .x(self.position.0)
            .y(self.position.1)
            .into();

        tile_component
    }

    fn render_tile_svg(&self) -> Component {
        if !Path::new(&self.path).exists() {
            println!("can't find tile: {}", self.path);
        }

        let mut svg = svg(self.path.clone());
        if matches!(self.direction, Direction::DOWN | Direction::UP) {
            svg = svg.width(self.size.width);
            svg = svg.height(self.size.height);
        } else {
            svg = svg.height(self.size.width);
            svg = svg.width(self.size.height);
        }
        container(svg).into()
    }
}

// Builder methods
impl TileRenderer {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            path: String::new(),
            size: Size {
                width: 0.,
                height: 0.,
            },
            direction: Direction::DOWN,
            position: (0., 0.),
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Size {
            width: size,
            height: size * TILE_ASPECT_RATIO,
        };

        self
    }

    pub fn with_direction(mut self, direction: &Direction) -> Self {
        self.direction = direction.clone();

        self
    }

    pub fn with_tile(mut self, tile: &GameTile) -> Self {
        self.path = TileRenderer::get_path_for_tile(&tile.data, &self.direction);
        self.id = tile.id.clone();

        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);

        self
    }
}

// Static methods
impl TileRenderer {
    pub fn get_path_for_tile(tile: &TileData, direction: &Direction) -> String {
        let tile_dir = match tile.suit {
            Suit::BAMBOO | Suit::MAN | Suit::TONG => "suits",
            Suit::DRAGON => "dragon",
            Suit::WIND => "wind",
            Suit::JOKER => "joker",
            Suit::FLOWER | Suit::SEASON => "bonus",
        };

        let lower_tile_name = tile.name.to_lowercase();
        let file_name = match tile.suit {
            Suit::BAMBOO | Suit::MAN | Suit::TONG => tile.code,
            Suit::DRAGON => match tile.name {
                "GREEN_DRAGON" => "fa",
                "RED_DRAGON" => "zhong",
                "WHITE_DRAGON" => "ban",
                _ => "",
            },
            Suit::WIND => &lower_tile_name.split("_").take(1).collect::<Vec<&str>>()[0],
            Suit::JOKER => "joker_baida",
            Suit::FLOWER | Suit::SEASON => {
                let slugs = &mut lower_tile_name.split("_").collect::<Vec<&str>>();
                slugs.reverse();
                slugs.join("-").leak()
            }
        };

        let suffix = match direction {
            Direction::DOWN => "",
            Direction::RIGHT => "_right",
            Direction::UP => "_up",
            Direction::LEFT => "_left",
        };

        let mut path = get_path(
            format!(
                "assets/tiles/oblique/{}/{}{}.svg",
                tile_dir, file_name, suffix
            )
            .as_str(),
        );

        if !Path::new(&path).exists() {
            match direction {
                Direction::LEFT => path = path.replace("_left", "_x"),
                Direction::RIGHT => path = path.replace("_right", "_x"),
                Direction::UP => path = path.replace("_up", ""),
                _ => {}
            }
        }

        path
    }
}
