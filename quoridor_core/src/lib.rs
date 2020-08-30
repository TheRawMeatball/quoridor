use tbmp_core::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wall {
    Horizontal(Position),
    Vertical(Position),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    PlaceWall(Wall),
    MovePawn(Position),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PawnMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quoridor {
    pub wall_counts: Vec<u8>,
    pub pawn_positions: Vec<Position>,
    pub walls: Vec<Wall>,
    pub turn_of: PlayerID,
    pub player_count: u8,
}


impl Game for Quoridor {

    type Move = Move;

    fn validate_move(&self, _: Move) -> Result<(), ()> {
        Ok(())
    }

    fn apply_move(&mut self, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                self.walls.push(wall);
            }
            Move::MovePawn(movement) => {
                self.pawn_positions[self.turn_of as usize] = movement;
            }
        }
        self.turn_of += 1;
        if self.turn_of == self.player_count {
            self.turn_of = 0;
        }
        MoveResult::Continue
    }

    fn default_board() -> Self {
        Quoridor {
            wall_counts: vec![10, 10],
            pawn_positions: vec![(4, 0).into(), (4, 8).into()],
            walls: vec![Wall::Horizontal((2, 2).into())],
            turn_of: 0,
            player_count: 2,
        }
    }

    fn player_count(&self) -> u8 {
        self.player_count
    }

    fn turn_of(&self) -> u8 {
        self.turn_of
    }
}