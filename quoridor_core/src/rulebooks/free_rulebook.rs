use crate::*;

#[derive(Clone)]
pub struct FreeQuoridor;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FreeQuoridorMetadata {
    pub turns_left: u8,
}

impl Rulebook for FreeQuoridor {
    const PLAYER_COUNT: u8 = 2;
    const PAWN_COUNT: u8 = 4;
    type Move = Move;
    type Metadata = FreeQuoridorMetadata;

    fn validate_move(_: &QGame<Self>, _: Move) -> Result<(), ()> {
        Ok(())
    }

    fn apply_move(game: &mut QGame<Self>, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                game.walls.insert(wall);
            }
            Move::MovePawn(start_pos, end_pos) => {
                let id = game.pawn_positions.remove_by_right(&start_pos).unwrap().0;
                game.pawn_positions.insert(id, end_pos);
            }
            Move::RemoveWall(wall) => {
                game.walls.remove(&wall);
            }
            Move::MoveWall(start, end) => {
                game.walls.remove(&start);
                game.walls.insert(end);
            }
        }
        match game.metadata.turns_left.checked_sub(1) {
            None => {
                game.metadata.turns_left = 1;
                game.turn_of += 1;
                if game.turn_of == Self::PLAYER_COUNT {
                    game.turn_of = 0;
                }
            }
            Some(i) => game.metadata.turns_left = i,
        }

        MoveResult::Continue
    }

    fn initial_server() -> QGame<Self> {
        let mut pawns = BiMap::with_capacity(4);
        pawns.insert(0, Position::from((2, 0)));
        pawns.insert(1, Position::from((6, 0)));
        pawns.insert(2, Position::from((2, 8)));
        pawns.insert(3, Position::from((6, 8)));
        QGame::<Self> {
            wall_counts: vec![10, 10],
            pawn_positions: pawns,
            walls: HashSet::new(),
            turn_of: 0,
            metadata: FreeQuoridorMetadata { turns_left: 1 },
        }
    }
}
