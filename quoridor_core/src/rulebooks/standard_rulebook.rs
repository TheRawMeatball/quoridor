use crate::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StandardQuoridor;

fn check_movable(
    game: &QGame<StandardQuoridor>,
    pawn_pos: Position,
    pos: Position,
    check_jump: bool,
) -> Result<(), ()> {
    let x = pawn_pos.x as i8 - pos.x as i8;
    let y = pawn_pos.y as i8 - pos.y as i8;

    if x.abs() + y.abs() == 1 {
        let orientation = if x.abs() > 0 {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };

        if game.walls.contains(&Wall {
            position: Position::from((pos.x + (x > 0) as u8, pos.y + (y > 0) as u8)),
            wall_type: WallType::Simple,
            orientation,
        }) || game.walls.contains(&Wall {
            position: Position::from((
                pos.x + (x > 0) as u8 + y.abs() as u8,
                pos.y + x.abs() as u8 + (y > 0) as u8,
            )),
            wall_type: WallType::Simple,
            orientation,
        }) {
            Err(())
        } else {
            Ok(())
        }
    } else if x.abs() + y.abs() == 2 && check_jump {
        if x.abs() == 2 || y.abs() == 2 {
            if game.pawn_positions.contains_right(&Position::from((
                (pawn_pos.x as i8 - x.signum()) as u8,
                (pawn_pos.y as i8 - y.signum()) as u8,
            ))) {
                check_movable(
                    game,
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
                && game
                    .pawn_positions
                    .contains_right(&Position::from((pawn_pos.x + 1, pawn_pos.y)))
                && (pawn_pos.x == 7
                    || check_movable(
                        game,
                        (pawn_pos.x + 1, pawn_pos.y).into(),
                        (pawn_pos.x + 2, pawn_pos.y).into(),
                        false,
                    )
                    .is_err())
                && check_movable(game, (pawn_pos.x + 1, pawn_pos.y).into(), pos, false).is_ok()
            {
                Ok(())
            } else if pawn_pos.x != 0
                && game
                    .pawn_positions
                    .contains_right(&Position::from((pawn_pos.x - 1, pawn_pos.y)))
                && (pawn_pos.x == 1
                    || check_movable(
                        game,
                        (pawn_pos.x - 1, pawn_pos.y).into(),
                        (pawn_pos.x - 2, pawn_pos.y).into(),
                        false,
                    )
                    .is_err())
                && check_movable(game, (pawn_pos.x - 1, pawn_pos.y).into(), pos, false).is_ok()
            {
                Ok(())
            } else if pawn_pos.y != 0
                && game
                    .pawn_positions
                    .contains_right(&Position::from((pawn_pos.x, pawn_pos.y - 1)))
                && (pawn_pos.y == 1
                    || check_movable(
                        game,
                        (pawn_pos.x, pawn_pos.y - 1).into(),
                        (pawn_pos.x, pawn_pos.y - 2).into(),
                        false,
                    )
                    .is_err())
                && check_movable(game, (pawn_pos.x, pawn_pos.y - 1).into(), pos, false).is_ok()
            {
                Ok(())
            } else if pawn_pos.y != 8
                && game
                    .pawn_positions
                    .contains_right(&Position::from((pawn_pos.x, pawn_pos.y + 1)))
                && (pawn_pos.y == 7
                    || check_movable(
                        game,
                        (pawn_pos.x, pawn_pos.y + 1).into(),
                        (pawn_pos.x, pawn_pos.y + 2).into(),
                        false,
                    )
                    .is_err())
                && check_movable(game, (pawn_pos.x, pawn_pos.y + 1).into(), pos, false).is_ok()
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
    game: &QGame<StandardQuoridor>,
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
        && check_movable(
            game,
            current_pos,
            (current_pos.x - 1, current_pos.y).into(),
            false,
        )
        .is_ok()
    {
        if let Ok(_) = check_reach(
            game,
            (current_pos.x - 1, current_pos.y).into(),
            target_pos,
            visited,
        ) {
            return Ok(());
        }
    }

    if current_pos.x != 8
        && !visited.contains(&(current_pos.x + 1, current_pos.y).into())
        && check_movable(
            game,
            current_pos,
            (current_pos.x + 1, current_pos.y).into(),
            false,
        )
        .is_ok()
    {
        if let Ok(_) = check_reach(
            game,
            (current_pos.x + 1, current_pos.y).into(),
            target_pos,
            visited,
        ) {
            return Ok(());
        }
    }

    if current_pos.y != 0
        && !visited.contains(&(current_pos.x, current_pos.y - 1).into())
        && check_movable(
            game,
            current_pos,
            (current_pos.x, current_pos.y - 1).into(),
            false,
        )
        .is_ok()
    {
        if let Ok(_) = check_reach(
            game,
            (current_pos.x, current_pos.y - 1).into(),
            target_pos,
            visited,
        ) {
            return Ok(());
        }
    }

    if current_pos.y != 8
        && !visited.contains(&(current_pos.x, current_pos.y + 1).into())
        && check_movable(
            game,
            current_pos,
            (current_pos.x, current_pos.y + 1).into(),
            false,
        )
        .is_ok()
    {
        if let Ok(_) = check_reach(
            game,
            (current_pos.x, current_pos.y + 1).into(),
            target_pos,
            visited,
        ) {
            return Ok(());
        }
    }

    Err(())
}

impl Rulebook for StandardQuoridor {
    const PLAYER_COUNT: u8 = 2;
    const PAWN_COUNT: u8 = 2;
    type Move = Move;
    type Metadata = ();

    fn validate_move(game: &QGame<Self>, qmove: Move) -> Result<(), ()> {
        match qmove {
            Move::PlaceWall(wall) => {
                if game.wall_counts[game.turn_of as usize] == 0 {
                    Err(())
                } else {
                    match wall {
                        Wall {
                            orientation: Orientation::Horizontal,
                            position: pos,
                            ..
                        } => {
                            if pos.x == 0 || pos.x == 9 || pos.y == 0 || pos.y == 9 {
                                Err(())
                            } else {
                                if !game.walls.contains(&Wall::vertical(pos))
                                    && !game.walls.contains(&Wall::horizontal(
                                        (pos.x.wrapping_sub(1), pos.y).into(),
                                    ))
                                    && !game.walls.contains(&Wall::horizontal(Position::from((
                                        pos.x + 1,
                                        pos.y,
                                    ))))
                                {
                                    let mut hypothetical = Clone::clone(game);
                                    hypothetical.walls.insert(wall);
                                    (0u8..9)
                                        .find(|x| {
                                            check_reach(
                                                &hypothetical,
                                                game.pawn_positions
                                                    .get_by_left(&0u8)
                                                    .unwrap()
                                                    .clone(),
                                                (*x, 8).into(),
                                                &mut HashSet::new(),
                                            )
                                            .is_ok()
                                                && check_reach(
                                                    &hypothetical,
                                                    game.pawn_positions
                                                        .get_by_left(&1u8)
                                                        .unwrap()
                                                        .clone(),
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
                        Wall {
                            orientation: Orientation::Vertical,
                            position: pos,
                            ..
                        } => {
                            if pos.x == 0 || pos.x == 9 || pos.y == 0 || pos.y == 9 {
                                Err(())
                            } else {
                                if !game.walls.contains(&Wall::horizontal(pos))
                                    && !game.walls.contains(&Wall::vertical(Position::from((
                                        pos.x,
                                        pos.y.wrapping_sub(1),
                                    ))))
                                    && !game.walls.contains(&Wall::vertical(Position::from((
                                        pos.x,
                                        pos.y + 1,
                                    ))))
                                {
                                    let mut hypothetical = Clone::clone(game);
                                    hypothetical.walls.insert(wall);
                                    (0u8..9)
                                        .find(|x| {
                                            check_reach(
                                                &hypothetical,
                                                game.pawn_positions
                                                    .get_by_left(&0u8)
                                                    .unwrap()
                                                    .clone(),
                                                (*x, 8).into(),
                                                &mut HashSet::new(),
                                            )
                                            .is_ok()
                                                && check_reach(
                                                    &hypothetical,
                                                    game.pawn_positions
                                                        .get_by_left(&1u8)
                                                        .unwrap()
                                                        .clone(),
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
            Move::MovePawn(start_pos, target_pos) => {
                if let Ok(_) = check_movable(game, start_pos, target_pos, true) {
                    if game.pawn_positions.contains_right(&target_pos) {
                        Err(())
                    } else {
                        Ok(())
                    }
                } else {
                    Err(())
                }
            }
            Move::RemoveWall(_) => Err(()),
            Move::MoveWall(_, _) => Err(()),
        }
    }

    fn apply_move(game: &mut QGame<Self>, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                game.wall_counts[game.turn_of as usize] -= 1;
                game.walls.insert(wall);
            }
            Move::MovePawn(start_pos, end_pos) => {
                let id = game.pawn_positions.remove_by_right(&start_pos).unwrap().0;
                game.pawn_positions.insert(id, end_pos);
            }
            Move::RemoveWall(_) => {}
            Move::MoveWall(_, _) => {}
        }
        game.turn_of += 1;
        if game.turn_of == Self::PLAYER_COUNT {
            game.turn_of = 0;
        }

        if game.pawn_positions.get_by_left(&0u8).unwrap().clone().y == 8 {
            MoveResult::Win(0)
        } else if game.pawn_positions.get_by_left(&1u8).unwrap().clone().y == 0 {
            MoveResult::Win(1)
        } else {
            MoveResult::Continue
        }
    }

    fn initial_server() -> QGame<Self> {
        let mut pawns = BiMap::with_capacity(2);
        pawns.insert(0, Position::from((4, 0)));
        pawns.insert(1, Position::from((4, 8)));
        QGame::<Self> {
            wall_counts: vec![10, 10],
            pawn_positions: pawns,
            walls: HashSet::new(),
            turn_of: 0,
            metadata: (),
        }
    }
}
