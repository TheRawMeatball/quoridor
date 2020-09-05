use crate::*;

pub fn setup(mut commands: Commands, board_mats: ResMut<BoardMaterials>) {
    let sector_count = 9.0 * WALL_TO_SPOT_RATIO + 10.0;

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // Square Node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                aspect_ratio: Some(1.0),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            //material: board_mats.base_mat_handle,
            ..Default::default()
        })
        .with_children(|parent| {
            let wall_func = |parent: &mut ChildBuilder, x: u8| {
                // First corner
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            size: Size::new(
                                Val::Percent(100.0),
                                Val::Percent(100.0 / sector_count),
                            ),
                            ..Default::default()
                        },
                        material: board_mats.wall_slot_mat_handle,
                        ..Default::default()
                    })
                    .with(BoardElement::WallSlot)
                    .with(Position { x, y: 0 });

                for y in 1..10 {
                    parent
                        // Wall
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(100.0),
                                    Val::Percent((100.0 / sector_count) * WALL_TO_SPOT_RATIO),
                                ),
                                ..Default::default()
                            },
                            material: board_mats.wall_slot_mat_handle,
                            ..Default::default()
                        })
                        .with(BoardElement::WallSlot)
                        .with(Wall::vertical(Position::from((x, y - 1))))
                        // Corner
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(100.0),
                                    Val::Percent(100.0 / sector_count),
                                ),
                                ..Default::default()
                            },
                            material: board_mats.wall_slot_mat_handle,
                            ..Default::default()
                        })
                        .with(BoardElement::WallSlot)
                        .with(Position { x, y });
                }
            };

            // First wall sector
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0 / sector_count), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    material: board_mats.wall_slot_mat_handle,
                    ..Default::default()
                })
                .with_children(|parent| wall_func(parent, 0));

            for x in 0..9 {
                parent
                    // Node sectors
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(
                                Val::Percent((100.0 / sector_count) * WALL_TO_SPOT_RATIO),
                                Val::Percent(100.0),
                            ),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // First wall
                        parent
                            .spawn(ButtonComponents {
                                style: Style {
                                    size: Size::new(
                                        Val::Percent(100.0),
                                        Val::Percent(100.0 / sector_count),
                                    ),
                                    ..Default::default()
                                },
                                material: board_mats.wall_slot_mat_handle,
                                ..Default::default()
                            })
                            .with(BoardElement::WallSlot)
                            .with(Wall::horizontal(Position::from((x, 0))));

                        for y in 0..9 {
                            parent
                                // Node
                                .spawn(ButtonComponents {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(100.0),
                                            Val::Percent(
                                                (100.0 / sector_count) * WALL_TO_SPOT_RATIO,
                                            ),
                                        ),
                                        ..Default::default()
                                    },
                                    material: board_mats.base_mat_handle,
                                    ..Default::default()
                                })
                                .with(BoardElement::EmptyNode)
                                .with(Position { x, y })
                                // Wall
                                .spawn(ButtonComponents {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(100.0),
                                            Val::Percent(100.0 / sector_count),
                                        ),
                                        ..Default::default()
                                    },
                                    material: board_mats.wall_slot_mat_handle,
                                    ..Default::default()
                                })
                                .with(BoardElement::WallSlot)
                                .with(Wall::horizontal(Position::from((x, y + 1))));
                        }
                    })
                    // Wall sectors
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(
                                Val::Percent(100.0 / sector_count),
                                Val::Percent(100.0),
                            ),
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| wall_func(parent, x + 1));
            }
        });
}
