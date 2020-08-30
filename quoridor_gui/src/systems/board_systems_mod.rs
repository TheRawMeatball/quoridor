use crate::*;

pub fn input_system(
    mut state: ResMut<BoardState>,
    board_materials: Res<BoardMaterials>,
    game: Res<Game>,
    side: Res<AgentSide>,
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
    for (_button, interaction, mut material, element_type, pos, _wall) in
        &mut interaction_query.iter()
    {
        match *interaction {
            Interaction::Clicked(flags) => {
                if let Some(pos) = pos {
                    if *element_type == BoardElement::WallSlot {
                        let event = MoveEvent(Move::PlaceWall({
                            if flags.check(MouseButton::Left) {
                                Wall::Vertical(*pos)
                            } else {
                                Wall::Horizontal(*pos)
                            }
                        }));
                        moves.send(event);
                        state.highlight_pawn = false;
                        state.can_highlight = true;
                    } else if *element_type == BoardElement::EmptyNode {
                        if (if *side == AgentSide::A {
                            &game.player_a_pawn_position
                        } else {
                            &game.player_b_pawn_position
                        }) == pos
                        {
                            println!("clicking pawn {:?}!", pos);
                            if state.can_highlight {
                                state.highlight_pawn = !state.highlight_pawn;
                                state.can_highlight = false;
                            }
                        } else if state.highlight_pawn {
                            let event = MoveEvent(Move::MovePawn(*pos));
                            moves.send(event);
                            state.highlight_pawn = false;
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
                    if *pos
                        == (if *side == AgentSide::A {
                            game.player_a_pawn_position
                        } else {
                            game.player_b_pawn_position
                        })
                    {
                        if !state.highlight_pawn {
                            //*material = board_materials.highlight;
                        }
                        println!("hovering pawn {:?}!", pos);
                        state.can_highlight = true;
                    } else {
                        *material = board_materials.highlight;
                    }
                } else {
                    *material = board_materials.highlight;
                }
            }
            Interaction::None => {
                if let Some(pos) = pos {
                    if *pos
                        == (if *side == AgentSide::A {
                            game.player_a_pawn_position
                        } else {
                            game.player_b_pawn_position
                        })
                    {
                        println!("not clicking pawn {:?}!", pos);
                        state.can_highlight = true;
                    }
                }
            }
        }
    }
}

pub fn board_update_system(
    state: Res<BoardState>,
    board_materials: Res<BoardMaterials>,
    game: Res<Game>,
    side: Res<AgentSide>,
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
            let second_wall_edge = match wall {
                Wall::Horizontal(p) => Wall::Horizontal((p.x + 1, p.y).into()),
                Wall::Vertical(p) => Wall::Vertical((p.x, p.y + 1).into()),
            };

            if game.walls.contains(wall) || game.walls.contains(&second_wall_edge) {
                *element_type = BoardElement::Wall;
            }
        }

        if let Some(pos) = pos {
            if let BoardElement::EmptyNode = *element_type {
            } else {
                if game.walls.contains(&Wall::Horizontal(*pos))
                    || game.walls.contains(&Wall::Vertical(*pos))
                {
                    *element_type = BoardElement::Wall;
                }
            }
        }

        *material = match *element_type {
            BoardElement::EmptyNode => {
                if &game.player_a_pawn_position == pos.unwrap() {
                    if *side == AgentSide::A && state.highlight_pawn {
                        board_materials.select
                    } else {
                        board_materials.pawn1_mat_handle
                    }
                } else if &game.player_b_pawn_position == pos.unwrap() {
                    if *side == AgentSide::B && state.highlight_pawn {
                        board_materials.select
                    } else {
                        board_materials.pawn2_mat_handle
                    }
                } else {
                    board_materials.base_mat_handle
                }
            }
            BoardElement::WallSlot => board_materials.wall_slot_mat_handle,
            BoardElement::Wall => board_materials.wall_mat_handle,
        };
    }
}
