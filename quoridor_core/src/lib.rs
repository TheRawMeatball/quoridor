use serde::{Deserialize, Serialize};
use std::collections::hash_set::HashSet;
use tbmp::*;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wall {
    Horizontal(Position),
    Vertical(Position),
}

impl Wall {
    fn pos(self) -> Position {
        match self {
            Wall::Horizontal(p) => p,
            Wall::Vertical(p) => p,
        }
    }
    fn with_orientation(self, o: Wall) -> Wall {
        match o {
            Wall::Horizontal(_) => Wall::Horizontal(self.pos()),
            Wall::Vertical(_) => Wall::Vertical(self.pos()),
        }
    }
    fn with_pos(self, p: Position) -> Wall {
        Wall::Vertical(p).with_orientation(self)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    PlaceWall(Wall),
    MovePawn(u8, Position),
}

pub mod standard_rulebook;
pub mod free_rulebook;