#![feature(min_const_generics)]

use bimap::BiMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
pub struct Wall {
    pub position: Position,
    pub wall_type: WallType,
    pub orientation: Orientation,
}

impl Wall {
    pub fn vertical(pos: Position) -> Self {
        Wall {
            position: pos,
            wall_type: WallType::Simple,
            orientation: Orientation::Vertical,
        }
    }

    pub fn horizontal(pos: Position) -> Self {
        Wall {
            position: pos,
            wall_type: WallType::Simple,
            orientation: Orientation::Horizontal,
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WallType {
    Simple,
    Single,
    Strong,
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    PlaceWall(Wall),
    RemoveWall(Wall),
    MoveWall(Wall, Wall),
    MovePawn(Position, Position),
}

pub type PawnID = u8;

pub trait QGTrait: Send + Sync {
    fn pawns(&self) -> &BiMap<PawnID, Position>;
    fn walls(&self) -> &HashSet<Wall>;
    fn turn_of(&self) -> PlayerID;
}

impl<Rb: Rulebook> QGTrait for QGame<Rb> {
    fn pawns(&self) -> &BiMap<PawnID, Position> {
        &self.pawn_positions
    }

    fn walls(&self) -> &HashSet<Wall> {
        &self.walls
    }

    fn turn_of(&self) -> u8 {
        self.turn_of
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QGame<Rb: Rulebook> {
    pub wall_counts: Vec<u8>, //Change this to [u8; Rb::PLAYER_COUNT] when compiler allows
    pub pawn_positions: BiMap<PawnID, Position>,
    pub walls: HashSet<Wall>,
    pub turn_of: PlayerID,
    #[serde(bound = "")]
    pub metadata: Rb::Metadata,
}

pub trait Rulebook: Send + Clone + 'static {
    const PLAYER_COUNT: u8;
    const PAWN_COUNT: u8;

    type Move: MoveTrait;
    type Metadata: Serialize + DeserializeOwned + Send + Sync + Clone;

    fn validate_move(game: &QGame<Self>, qmove: Self::Move) -> Result<(), ()>;

    fn apply_move(game: &mut QGame<Self>, qmove: Self::Move) -> MoveResult;

    fn initial_server() -> QGame<Self>;
}

impl<Rb: Rulebook> Game for QGame<Rb> {
    type Move = Rb::Move;

    const PLAYER_COUNT: u8 = Rb::PLAYER_COUNT;

    fn validate_move(&self, qmove: Self::Move) -> Result<(), ()> {
        Rb::validate_move(self, qmove)
    }

    fn apply_move(&mut self, qmove: Self::Move) -> MoveResult {
        Rb::apply_move(self, qmove)
    }

    fn initial_server() -> Self {
        Rb::initial_server()
    }

    fn turn_of(&self) -> u8 {
        self.turn_of
    }
}

pub trait MoveTrait: Copy + Send + Serialize + DeserializeOwned {}
impl MoveTrait for Move {}

pub mod rulebooks;

#[macro_export]
macro_rules! generate_rulebook {
    ($($rulebook_ident:ident),*) => {
        pub(crate) enum Quoridor {
            $(
                $rulebook_ident(QGame<$rulebook_ident>),
            )*
        }

        pub(crate) enum QAgent {
            $(
                $rulebook_ident(AgentCore<QGame<$rulebook_ident>>),
            )*
        }

        pub(crate) enum RulebookMove {
            $(
                $rulebook_ident(<$rulebook_ident as Rulebook>::Move),
            )*
        }

        pub(crate) enum QGameEvent {
            GameStart(Quoridor, PlayerID),
            MoveHappened(RulebookMove),
            YourTurn,
            ValidMove,
            InvalidMove,
            OpponentQuit,
            GameEnd(Option<PlayerID>),
        }

        impl Quoridor {
            pub(crate) fn get_pawn_count(&self) -> u8 {
                match self {
                    $(
                        Self::$rulebook_ident(_) => $rulebook_ident::PAWN_COUNT,
                    )*
                }
            }

            pub(crate) fn get_player_count(&self) -> u8 {
                match self {
                    $(
                        Self::$rulebook_ident(_) => $rulebook_ident::PLAYER_COUNT,
                    )*
                }
            }

            pub(crate) fn apply_move(&mut self, qmove: &RulebookMove) {
                match self {
                    $(
                        Self::$rulebook_ident(g) => { g.apply_move(match qmove {
                            RulebookMove::$rulebook_ident(qmv) => *qmv,
                            _ => unreachable!(),
                        }); },
                    )*
                }
            }
        }

        impl QAgent {
            pub(crate) fn recv_event(&self) -> Result<QGameEvent, Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => Ok(match c.event_channel.try_recv()? {
                            GameEvent::GameStart(g, id) => QGameEvent::GameStart(Quoridor::$rulebook_ident(g), id),
                            GameEvent::MoveHappened(qmove) => QGameEvent::MoveHappened(RulebookMove::$rulebook_ident(qmove)),
                            GameEvent::YourTurn => QGameEvent::YourTurn,
                            GameEvent::ValidMove => QGameEvent::ValidMove,
                            GameEvent::InvalidMove => QGameEvent::InvalidMove,
                            GameEvent::OpponentQuit => QGameEvent::OpponentQuit,
                            GameEvent::GameEnd(id) => QGameEvent::GameEnd(id),
                        }),
                    )*
                }
            }

            pub(crate) fn send_move(&self, qmove: RulebookMove) -> Result<(), Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => c.move_channel.send(match qmove {
                            RulebookMove::$rulebook_ident(qmv) => qmv,
                            _ => unreachable!(),
                        })?,
                    )*
                };
                Ok(())
            }
        }

        impl RulebookMove {
            pub(crate) fn wrap(game: &Quoridor, qmove: &Move) -> Self {
                match game {
                    $(
                        Quoridor::$rulebook_ident(_) => RulebookMove::$rulebook_ident(*qmove),
                    )*
                }
            }
        }

        impl QGTrait for Quoridor {
            fn pawns(&self) -> &BiMap<PawnID, Position> {
                match self {
                    $(
                        Self::$rulebook_ident(g) => &g.pawn_positions,
                    )*
                }
            }

            fn walls(&self) -> &std::collections::HashSet<Wall> {
                match self {
                    $(
                        Self::$rulebook_ident(g) => &g.walls,
                    )*
                }
            }

            fn turn_of(&self) -> PlayerID {
                match self {
                    $(
                        Self::$rulebook_ident(g) => g.turn_of,
                    )*
                }
            }
        }
    };
}
