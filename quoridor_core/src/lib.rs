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

#[derive(Clone, Serialize, Deserialize)]
pub struct Quoridor {
    pub wall_counts: [u8; 2],
    pub pawn_positions: [Position; 2],
    pub walls: HashSet<Wall>,
    pub turn_of: PlayerID,
    pub player_count: u8,
}

impl Quoridor {
    fn check_movable(&self, pawn_pos: Position, pos: Position, check_jump: bool) -> Result<(), ()> {
        let x = pawn_pos.x as i8 - pos.x as i8;
        let y = pawn_pos.y as i8 - pos.y as i8;

        if x.abs() + y.abs() == 1 {
            let wall_type = if x.abs() > 0 {
                Wall::Vertical((0, 0).into())
            } else {
                Wall::Horizontal((0, 0).into())
            };

            if self.walls.contains(
                &wall_type.with_pos((pos.x + (x > 0) as u8, pos.y + (y > 0) as u8).into()),
            ) || self.walls.contains(
                &wall_type.with_pos(
                    (
                        pos.x + (x > 0) as u8 + y.abs() as u8,
                        pos.y + x.abs() as u8 + (y > 0) as u8,
                    )
                        .into(),
                ),
            ) {
                Err(())
            } else {
                Ok(())
            }
        } else if x.abs() + y.abs() == 2 && check_jump {
            if x.abs() == 2 || y.abs() == 2 {
                if self.pawn_positions.contains(
                    &(
                        (pawn_pos.x as i8 - x.signum()) as u8,
                        (pawn_pos.y as i8 - y.signum()) as u8,
                    )
                        .into(),
                ) {
                    self.check_movable(
                        (
                            (pawn_pos.x as i8 - x.signum()) as u8,
                            (pawn_pos.y as i8 - y.signum()) as u8,
                        )
                            .into(),
                        pos,
                        false,
                    )
                } else {
                    Err(())
                }
            } else {
                if pawn_pos.x != 8
                    && self
                        .pawn_positions
                        .contains(&(pawn_pos.x + 1, pawn_pos.y).into())
                    && (pawn_pos.x == 7
                        || self
                            .check_movable(
                                (pawn_pos.x + 1, pawn_pos.y).into(),
                                (pawn_pos.x + 2, pawn_pos.y).into(),
                                false,
                            )
                            .is_err())
                    && self
                        .check_movable((pawn_pos.x + 1, pawn_pos.y).into(), pos, false)
                        .is_ok()
                {
                    Ok(())
                } else if pawn_pos.x != 0
                    && self
                        .pawn_positions
                        .contains(&(pawn_pos.x - 1, pawn_pos.y).into())
                    && (pawn_pos.x == 1
                        || self
                            .check_movable(
                                (pawn_pos.x - 1, pawn_pos.y).into(),
                                (pawn_pos.x - 2, pawn_pos.y).into(),
                                false,
                            )
                            .is_err())
                    && self
                        .check_movable((pawn_pos.x - 1, pawn_pos.y).into(), pos, false)
                        .is_ok()
                {
                    Ok(())
                } else if pawn_pos.y != 0
                    && self
                        .pawn_positions
                        .contains(&(pawn_pos.x, pawn_pos.y - 1).into())
                    && (pawn_pos.y == 1
                        || self
                            .check_movable(
                                (pawn_pos.x, pawn_pos.y - 1).into(),
                                (pawn_pos.x, pawn_pos.y - 2).into(),
                                false,
                            )
                            .is_err())
                    && self
                        .check_movable((pawn_pos.x, pawn_pos.y - 1).into(), pos, false)
                        .is_ok()
                {
                    Ok(())
                } else if pawn_pos.y != 8
                    && self
                        .pawn_positions
                        .contains(&(pawn_pos.x, pawn_pos.y + 1).into())
                    && (pawn_pos.y == 7
                        || self
                            .check_movable(
                                (pawn_pos.x, pawn_pos.y + 1).into(),
                                (pawn_pos.x, pawn_pos.y + 2).into(),
                                false,
                            )
                            .is_err())
                    && self
                        .check_movable((pawn_pos.x, pawn_pos.y + 1).into(), pos, false)
                        .is_ok()
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
        } else {
            Err(())
        }
    }

    fn check_reach(
        &self,
        current_pos: Position,
        target_pos: Position,
        visited: &mut HashSet<Position>,
    ) -> Result<(), ()> {
        visited.insert(current_pos);
        //println!("visiting {:?}", current_pos);
        //println!("visited: {:?}", visited);
        if current_pos == target_pos {
            return Ok(());
        }

        if current_pos.x != 0
            && !visited.contains(&(current_pos.x - 1, current_pos.y).into())
            && self
                .check_movable(
                    current_pos,
                    (current_pos.x - 1, current_pos.y).into(),
                    false,
                )
                .is_ok()
        {
            if let Ok(_) = self.check_reach(
                (current_pos.x - 1, current_pos.y).into(),
                target_pos,
                visited,
            ) {
                return Ok(());
            }
        }

        if current_pos.x != 8
            && !visited.contains(&(current_pos.x + 1, current_pos.y).into())
            && self
                .check_movable(
                    current_pos,
                    (current_pos.x + 1, current_pos.y).into(),
                    false,
                )
                .is_ok()
        {
            if let Ok(_) = self.check_reach(
                (current_pos.x + 1, current_pos.y).into(),
                target_pos,
                visited,
            ) {
                return Ok(());
            }
        }

        if current_pos.y != 0
            && !visited.contains(&(current_pos.x, current_pos.y - 1).into())
            && self
                .check_movable(
                    current_pos,
                    (current_pos.x, current_pos.y - 1).into(),
                    false,
                )
                .is_ok()
        {
            if let Ok(_) = self.check_reach(
                (current_pos.x, current_pos.y - 1).into(),
                target_pos,
                visited,
            ) {
                return Ok(());
            }
        }

        if current_pos.y != 8
            && !visited.contains(&(current_pos.x, current_pos.y + 1).into())
            && self
                .check_movable(
                    current_pos,
                    (current_pos.x, current_pos.y + 1).into(),
                    false,
                )
                .is_ok()
        {
            if let Ok(_) = self.check_reach(
                (current_pos.x, current_pos.y + 1).into(),
                target_pos,
                visited,
            ) {
                return Ok(());
            }
        }

        Err(())
    }
}

impl Game for Quoridor {
    const PLAYER_COUNT: u8 = 2;
    type Move = Move;

    fn validate_move(&self, qmove: Move) -> Result<(), ()> {
        match qmove {
            Move::PlaceWall(wall) => {
                if self.wall_counts[self.turn_of as usize] > 0 {
                    Err(())
                } else {
                    match wall {
                        Wall::Horizontal(pos) => {
                            if pos.x == 0 || pos.x == 9 || pos.y == 0 || pos.y == 9 {
                                Err(())
                            } else {
                                if !self.walls.contains(&Wall::Vertical(pos))
                                    && !self.walls.contains(&Wall::Horizontal(
                                        (pos.x.wrapping_sub(1), pos.y).into(),
                                    ))
                                    && !self
                                        .walls
                                        .contains(&Wall::Horizontal((pos.x + 1, pos.y).into()))
                                {
                                    let mut hypothetical = Clone::clone(self);
                                    hypothetical.walls.insert(wall);
                                    (0u8..9)
                                        .find(|x| {
                                            hypothetical
                                                .check_reach(
                                                    self.pawn_positions[0],
                                                    (*x, 8).into(),
                                                    &mut HashSet::new(),
                                                )
                                                .is_ok()
                                                && hypothetical
                                                    .check_reach(
                                                        self.pawn_positions[1],
                                                        (*x, 0).into(),
                                                        &mut HashSet::new(),
                                                    )
                                                    .is_ok()
                                        })
                                        .map(|_| ())
                                        .ok_or(())
                                } else {
                                    Err(())
                                }
                            }
                        }
                        Wall::Vertical(pos) => {
                            if pos.x == 0 || pos.x == 9 || pos.y == 0 || pos.y == 9 {
                                Err(())
                            } else {
                                if !self.walls.contains(&Wall::Horizontal(pos))
                                    && !self.walls.contains(&Wall::Vertical(
                                        (pos.x, pos.y.wrapping_sub(1)).into(),
                                    ))
                                    && !self
                                        .walls
                                        .contains(&Wall::Vertical((pos.x, pos.y + 1).into()))
                                {
                                    let mut hypothetical = Clone::clone(self);
                                    hypothetical.walls.insert(wall);
                                    (0u8..9)
                                        .find(|x| {
                                            hypothetical
                                                .check_reach(
                                                    self.pawn_positions[0],
                                                    (*x, 8).into(),
                                                    &mut HashSet::new(),
                                                )
                                                .is_ok()
                                                && hypothetical
                                                    .check_reach(
                                                        self.pawn_positions[1],
                                                        (*x, 0).into(),
                                                        &mut HashSet::new(),
                                                    )
                                                    .is_ok()
                                        })
                                        .map(|_| ())
                                        .ok_or(())
                                } else {
                                    Err(())
                                }
                            }
                        }
                    }
                }
            }
            Move::MovePawn(pawn_id, pos) => {
                let pawn_pos = self.pawn_positions[pawn_id as usize];
                if let Ok(_) = self.check_movable(pawn_pos, pos, true) {
                    if self.pawn_positions.contains(&pos) {
                        Err(())
                    } else {
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

        if self.pawn_positions[0].y == 8 {
            MoveResult::Win(0)
        } else if self.pawn_positions[1].y == 0 {
            MoveResult::Win(1)
        } else {
            MoveResult::Continue
        }
    }

    fn initial_server() -> Self {
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
