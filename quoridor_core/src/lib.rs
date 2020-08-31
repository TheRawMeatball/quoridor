use serde::{Deserialize, Serialize};
use tbmp::*;
use std::collections::hash_set::HashSet;

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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    PlaceWall(Wall),
    MovePawn(u8, Position),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quoridor {
    pub wall_counts: [u8; 2],
    pub pawn_positions: [Position; 2],
    pub walls: HashSet<Wall>,
    pub turn_of: PlayerID,
    pub player_count: u8,
}

impl Quoridor {
    fn check_movable(
        &self,
        pawn_pos: Position,
        pos: Position,
    ) -> Result<(), ()> {
        match (
            pawn_pos.x as i8 - pos.x as i8,
            pawn_pos.y as i8 - pos.y as i8,
        ) {
            (x, 0) => {
                if x.abs() > 1 || x == 0 {
                    Err(())
                } else {
                    if self
                        .walls
                        .contains(&Wall::Vertical((pos.x + (x > 0) as u8, pos.y).into()))
                        || self
                            .walls
                            .contains(&Wall::Vertical((pos.x + (x > 0) as u8, pos.y + 1).into()))
                    {
                        Err(())
                    } else {
                        Ok(())
                    }
                }
            }
            (0, y) => {
                if y.abs() > 1 || y == 0 {
                    Err(())
                } else {
                    if self
                        .walls
                        .contains(&Wall::Horizontal((pos.x, pos.y + (y > 0) as u8).into()))
                        || self
                            .walls
                            .contains(&Wall::Horizontal((pos.x + 1, pos.y + (y > 0) as u8).into()))
                    {
                        Err(())
                    } else {
                        Ok(())
                    }
                }
            }
            _ => Err(()),
        }
    }
}

impl Game for Quoridor {
    const PLAYER_COUNT: u8 = 2;
    type Move = Move;

    fn validate_move(&self, qmove: Move) -> Result<(), ()> {
        match qmove {
            Move::PlaceWall(wall) => match wall {
                Wall::Horizontal(pos) => {
                    if pos.x == 0 || pos.x == 8 || pos.y == 0 || pos.y == 8 {
                        Err(())
                    } else {
                        if !self.walls.contains(&Wall::Vertical(pos))
                            && !self
                                .walls
                                .contains(&Wall::Horizontal((pos.x.wrapping_sub(1), pos.y).into()))
                            && !self
                                .walls
                                .contains(&Wall::Horizontal((pos.x + 1, pos.y).into()))
                        {
                            Ok(())
                        } else {
                            Err(())
                        }
                    }
                }
                Wall::Vertical(pos) => {
                    if pos.x == 0 || pos.x == 8 || pos.y == 0 || pos.y == 8 {
                        Err(())
                    } else {
                        if !self.walls.contains(&Wall::Horizontal(pos))
                            && !self
                                .walls
                                .contains(&Wall::Vertical((pos.x, pos.y.wrapping_sub(1)).into()))
                            && !self
                                .walls
                                .contains(&Wall::Vertical((pos.x, pos.y + 1).into()))
                        {
                            Ok(())
                        } else {
                            Err(())
                        }
                    }
                }
            },
            Move::MovePawn(pawn_id, pos) => {
                let pawn_pos = self.pawn_positions[pawn_id as usize];
                if let Ok(_) = self.check_movable(pawn_pos, pos)
                {
                    if self.pawn_positions.contains(&pawn_pos) {
                        Err(())
                    }
                    else {
                        //TODO: PATHFINDING

                        Ok(())
                    }
                } else {
                    Err(())
                }
            }
        }
    }

    fn apply_move(&mut self, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                self.walls.insert(wall);
            }
            Move::MovePawn(pawn_id, movement) => {
                self.pawn_positions[pawn_id as usize] = movement;
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
            wall_counts: [10, 10],
            pawn_positions: [(4, 0).into(), (4, 8).into()],
            walls: HashSet::new(),
            turn_of: 0,
            player_count: 2,
        }
    }

    fn turn_of(&self) -> u8 {
        self.turn_of
    }
}
