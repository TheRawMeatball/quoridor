#![feature(min_const_generics)]

use bimap::BiMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::hash_set::HashSet;
use tbmp_core::*;

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