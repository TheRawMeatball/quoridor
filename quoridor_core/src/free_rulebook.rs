use super::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FreeQuoridor {
    pub wall_counts: [u8; 2],
    pub pawn_positions: [Position; 4],
    pub walls: HashSet<Wall>,
    pub turn_of: PlayerID,
    pub turns_left: u8,
}

impl Game for FreeQuoridor {
    const PLAYER_COUNT: u8 = 2;
    type Move = Move;

    fn validate_move(&self, _: Move) -> Result<(), ()> {
        Ok(())
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
        match self.turns_left.checked_sub(1) {
            None => {
                self.turns_left = 1;
                self.turn_of += 1;
                if self.turn_of == Self::PLAYER_COUNT {
                    self.turn_of = 0;
                }
            }
            Some(i) => self.turns_left = i,
        }

        MoveResult::Continue
    }

    fn initial_server() -> Self {
        FreeQuoridor {
            wall_counts: [10, 10],
            pawn_positions: [(2, 0).into(), (6, 0).into(), (2, 8).into(), (6, 8).into()],
            walls: HashSet::new(),
            turn_of: 0,
            turns_left: 1,
        }
    }

    fn turn_of(&self) -> u8 {
        self.turn_of
    }
}
