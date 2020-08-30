use crossbeam_channel::{self, Receiver, Sender};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::vec::Vec;

#[derive(Clone, Serialize, Deserialize)]
pub struct Game<Rb: Rulebook> {
    pub wall_counts: Vec<u8>,
    pub pawn_positions: Vec<Position>,
    pub walls: Vec<Wall>,
    pub turn_of: PlayerID,
    pub player_count: u8,
    pub metadata: Rb::Metadata,
}

#[derive(Clone)]
pub struct AgentCore<Rb: Rulebook> {
    pub move_channel: Sender<Move>,
    pub event_channel: Receiver<QuoridorEvent<Rb>>,
}

struct AntiCore<Rb: Rulebook> {
    pub move_channel: Receiver<Move>,
    pub event_channel: Sender<QuoridorEvent<Rb>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wall {
    Horizontal(Position),
    Vertical(Position),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    PlaceWall(Wall),
    MovePawn(Position),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PawnMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize)]
pub enum QuoridorEvent<Rb: Rulebook> {
    #[serde(bound = "")]
    GameStart(Game<Rb>, PlayerID),
    MoveHappened(Move),
    YourTurn,
    ValidMove,
    InvalidMove,
    OpponentQuit,
    GameEnd(Option<PlayerID>),
}

pub type PlayerID = u8;

pub enum MoveResult {
    Continue,
    Draw,
    Win(u8),
}

pub trait Rulebook : Serialize + DeserializeOwned + Send + Sized + Clone + 'static {
    type Metadata : Serialize + DeserializeOwned + Send + Clone;
    fn validate_move(game: &Game<Self>, qmove: Move) -> Result<(), ()>;
    fn apply_move(game: &mut Game<Self>, qmove: Move) -> MoveResult;
    fn default_board() -> Game<Self>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FreeRulebook();

impl Rulebook for FreeRulebook {
    type Metadata = ();
    fn validate_move(_: &Game<Self>, _: Move) -> Result<(), ()> {
        Ok(())
    }

    fn apply_move(game: &mut Game<Self>, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                game.walls.push(wall);
            }
            Move::MovePawn(movement) => {
                game.pawn_positions[game.turn_of as usize] = movement;
            }
        }
        game.turn_of += 1;
        if game.turn_of == game.player_count {
            game.turn_of = 0;
        }
        MoveResult::Continue
    }

    fn default_board() -> Game<Self> {
        Game {
            wall_counts: vec![10, 10],
            pawn_positions: vec![(4, 0).into(), (4, 8).into()],
            walls: vec![Wall::Horizontal((2, 2).into())],
            turn_of: 0,
            player_count: 2,
            metadata: (),
        }
    }
}

pub fn new_game<Rb: Rulebook>() -> Vec<AgentCore<Rb>> {
    let mut game = Rb::default_board();

    let channels = (0..game.player_count).fold(vec![], |mut tuples, _| {
        tuples.push((
            crossbeam_channel::unbounded::<Move>(),
            crossbeam_channel::unbounded::<QuoridorEvent<Rb>>(),
        ));
        tuples
    });

    let (cores, anti_cores) = channels
        .into_iter()
        .fold((vec![], vec![]), |mut vecs, channels| {
            vecs.0.push(AgentCore {
                move_channel: (channels.0).0,
                event_channel: (channels.1).1,
            });
            vecs.1.push(AntiCore {
                move_channel: (channels.0).1,
                event_channel: (channels.1).0,
            });
            vecs
        });

    let mut main_thread = move || -> Result<(), Box<dyn std::error::Error>> {
        use QuoridorEvent::*;

        for i in 0..game.player_count {
            anti_cores[i as usize]
                .event_channel
                .send(GameStart(Clone::clone(&game), i))
                .unwrap();
        }

        anti_cores[game.turn_of as usize]
            .event_channel
            .send(YourTurn)
            .unwrap();

        
            
        loop {
            let qmove = anti_cores[game.turn_of as usize]
                .move_channel
                .recv()?;

            if let Ok(()) = Rb::validate_move(&game, qmove) {
                let current_player = game.turn_of;
                for i in 0..game.player_count
                {
                    anti_cores[i as usize]
                        .event_channel
                        .send(MoveHappened(qmove))?;
                }
                match Rb::apply_move(&mut game, qmove) {
                    MoveResult::Continue => {}
                    MoveResult::Draw => {
                        for i in 0..game.player_count {
                            anti_cores[i as usize]
                                .event_channel
                                .send(GameEnd(None))?;
                        }
                        break;
                    }
                    MoveResult::Win(side) => {
                        for i in 0..game.player_count {
                            anti_cores[i as usize]
                                .event_channel
                                .send(GameEnd(Some(side)))?;
                        }
                        break;
                    }
                }

                anti_cores[current_player as usize]
                    .event_channel
                    .send(ValidMove)?;
                anti_cores[game.turn_of as usize]
                    .event_channel
                    .send(YourTurn)?;
            } else {
                anti_cores[game.turn_of as usize]
                    .event_channel
                    .send(InvalidMove)?;
                continue;
            }
        }
        
        Ok(())
    };

    std::thread::spawn(move ||
    {
        if let Err(e) = main_thread() {
            println!("{:?}", e);
        } else {

        }
    });

    cores
}
