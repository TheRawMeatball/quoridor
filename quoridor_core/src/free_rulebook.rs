use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct FreeQuoridor {
    pub wall_counts: [u8; 2],
    pub pawn_positions: [Position; 2],
    pub walls: HashSet<Wall>,
    pub turn_of: PlayerID,
    pub player_count: u8,
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
        self.turn_of += 1;
        if self.turn_of == self.player_count {
            self.turn_of = 0;
        }

        MoveResult::Continue
    }

    fn initial_server() -> Self {
        FreeQuoridor {
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
