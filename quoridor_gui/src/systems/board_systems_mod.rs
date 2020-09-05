use crate::*;

pub(crate) fn input_system(
    mut state: ResMut<BoardState>,
    board_materials: Res<BoardMaterials>,
    game: Res<Quoridor>,
    side: Res<u8>,
    mut moves: ResMut<Events<MoveEvent>>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &BoardElement,
        Option<&Position>,
        Option<&Wall>,
    )>,
) {
    //println!("{:?}", *state);
    for (_button, interaction, mut material, element_type, pos, wall) in
        &mut interaction_query.iter()
    {
        match *interaction {
            Interaction::Clicked(flags) => {
                if let Some(pos) = pos {
                    if *element_type == BoardElement::WallSlot {
                        let event = MoveEvent(Move::PlaceWall({
                            if flags.check(MouseButton::Left) {
                                Wall::vertical(*pos)
                            } else {
                                Wall::horizontal(*pos)
                            }
                        }));
                        moves.send(event);
                        state.highlight_pawn = None;
                        state.can_highlight = true;
                    } else if *element_type == BoardElement::EmptyNode {
                        if owned_pawn_check(*side, &game, *pos) {
                            if state.can_highlight {
                                state.highlight_pawn =
                                    if let Some(highlight_pos) = state.highlight_pawn {
                                        if highlight_pos == *pos {
                                            None
                                        } else {
                                            Some(*pos)
                                        }
                                    } else {
                                        Some(*pos)
                                    };
                                state.can_highlight = false;
                            }
                        } else if state.highlight_pawn.is_some() {
                            let event =
                                MoveEvent(Move::MovePawn(state.highlight_pawn.unwrap(), *pos));
                            moves.send(event);
                            state.highlight_pawn = None;
                            state.can_highlight = true;
                        }
                    }
                }
                //if let Some(wall) = wall {
                //    println!("{:?}", wall);
                //}
            }
            Interaction::Hovered => {
                if let Some(pos) = pos {
                    if owned_pawn_check(*side, &game, *pos) {
                        state.can_highlight = true;
                    } else {
                        match element_type {
                            BoardElement::WallSlot => {}
                            BoardElement::Wall => {}
                            BoardElement::EmptyNode => {
                                if state.highlight_pawn.is_some() {
                                    *material = board_materials.highlight;
                                }
                            }
                        }
                    }
                } else if let Some(_wall) = wall {
                    //*material = board_materials.highlight;
                }
            }
            Interaction::None => {
                if let Some(pos) = pos {
                    if owned_pawn_check(*side, &game, *pos) {
                        state.can_highlight = true;
                    }
                }
            }
        }
    }
}

pub(crate) fn board_update_system(
    state: Res<BoardState>,
    board_materials: Res<BoardMaterials>,
    game: Res<Quoridor>,
    mut query: Query<(
        &Button,
        &mut Handle<ColorMaterial>,
        Mut<BoardElement>,
        Option<&Position>,
        Option<&Wall>,
    )>,
) {
    //println!("{:?}", *state);
    for (_button, mut material, mut element_type, pos, wall) in &mut query.iter() {
        if let Some(wall) = wall {
            let mut second_wall_edge = wall.clone();
            second_wall_edge.position = Position::from((
                wall.position.x + (wall.orientation == Orientation::Horizontal) as u8,
                wall.position.y + (wall.orientation == Orientation::Vertical) as u8,
            ));

            if game.walls().contains(wall) || game.walls().contains(&second_wall_edge) {
                *element_type = BoardElement::Wall;
            }
        }

        if let Some(pos) = pos {
            if let BoardElement::EmptyNode = *element_type {
            } else {
                if game.walls().contains(&Wall::horizontal(*pos))
                    || game.walls().contains(&Wall::vertical(*pos))
                {
                    *element_type = BoardElement::Wall;
                }
            }
        }

        *material = match *element_type {
            BoardElement::EmptyNode => {
                if let Some(pos) = pos {
                    if let Some(id) = game.pawns().get_by_right(pos) {
                        if let Some(p) = state.highlight_pawn {
                            if p == *pos {
                                board_materials.select
                            } else {
                                board_materials.pawn_materials[*id as usize]
                            }
                        } else {
                            board_materials.pawn_materials[*id as usize]
                        }
                    } else {
                        board_materials.base_mat_handle
                    }
                } else {
                    unreachable!()
                }
            }
            BoardElement::WallSlot => board_materials.wall_slot_mat_handle,
            BoardElement::Wall => board_materials.wall_mat_handle,
        };
    }
}

fn owned_pawn_check(side: PlayerID, game: &Res<Quoridor>, pos: Position) -> bool {
    let pawns_per_player = game.get_pawn_count() / game.get_player_count();
    if let Some(&id) = game.pawns().get_by_right(&pos) {
        let min_id = side * pawns_per_player;
        let max_id = (side + 1) * pawns_per_player;
        min_id <= id && id < max_id
    } else {
        false
    }
}
