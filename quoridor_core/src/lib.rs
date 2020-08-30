use crossbeam_channel::{self, Receiver, Sender};
use nanoserde::{DeBin, SerBin};
use std::vec::Vec;

#[derive(Clone, SerBin, DeBin)]
pub struct Game {
    pub player_a_walls: u8,
    pub player_b_walls: u8,
    pub player_a_pawn_position: Position,
    pub player_b_pawn_position: Position,
    pub walls: Vec<Wall>,
    pub turn_of: AgentSide,
}

pub struct AgentCore {
    pub move_channel: Sender<Move>,
    pub event_channel: Receiver<QuoridorEvent>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, SerBin, DeBin)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Position {
    fn from((x, y): (u8, u8)) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, SerBin, DeBin)]
pub enum Wall {
    Horizontal(Position),
    Vertical(Position),
}

#[derive(Copy, Clone, Debug, SerBin, DeBin)]
pub enum Move {
    PlaceWall(Wall),
    MovePawn(Position),
}

#[derive(Copy, Clone, Debug, SerBin, DeBin)]
pub enum PawnMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(SerBin, DeBin)]
pub enum QuoridorEvent {
    GameStart(Game, AgentSide),
    YourTurn(Game),
    ValidMove(Game),
    InvalidMove,
    OpponentQuit,
    GameEnd(Option<AgentSide>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, SerBin, DeBin)]
pub enum AgentSide {
    A,
    B,
}

pub enum MoveResult {
    Continue,
    Draw,
    Win(AgentSide),
}

pub trait Rulebook {
    fn validate_move(game: &Game, qmove: Move) -> Result<(), ()>;
    fn apply_move(game: &mut Game, qmove: Move) -> MoveResult;
    fn default_board() -> Game {
        Game {
            player_a_walls: 0,
            player_b_walls: 0,
            player_a_pawn_position: (4, 0).into(),
            player_b_pawn_position: (4, 8).into(),
            walls: vec![Wall::Horizontal((2, 2).into())],
            turn_of: AgentSide::A,
        }
    }
}

pub struct FreeRulebook();
impl Rulebook for FreeRulebook {
    fn validate_move(_: &Game, _: Move) -> Result<(), ()> {
        Ok(())
    }

    fn apply_move(game: &mut Game, qmove: Move) -> MoveResult {
        match qmove {
            Move::PlaceWall(wall) => {
                game.walls.push(wall);
            }
            Move::MovePawn(movement) => {
                if game.turn_of == AgentSide::A {
                    game.player_a_pawn_position = movement;
                } else {
                    game.player_b_pawn_position = movement;
                }
            }
        }
        game.turn_of = if game.turn_of == AgentSide::A {
            AgentSide::B
        } else {
            AgentSide::A
        };
        MoveResult::Continue
    }
}

pub fn new_game<Rb: Rulebook>() -> (AgentCore, AgentCore) {
    let (agent_a_move_send, agent_a_move_recv) = crossbeam_channel::unbounded::<Move>();
    let (agent_b_move_send, agent_b_move_recv) = crossbeam_channel::unbounded::<Move>();
    let (agent_a_event_send, agent_a_event_recv) = crossbeam_channel::unbounded::<QuoridorEvent>();
    let (agent_b_event_send, agent_b_event_recv) = crossbeam_channel::unbounded::<QuoridorEvent>();

    std::thread::spawn(move || {
        use QuoridorEvent::*;

        let mut game = Rb::default_board();

        agent_a_event_send
            .send(GameStart(Clone::clone(&game), AgentSide::A))
            .unwrap();
        agent_b_event_send
            .send(GameStart(Clone::clone(&game), AgentSide::B))
            .unwrap();

        agent_a_event_send
            .send(YourTurn(Clone::clone(&game)))
            .unwrap();

        loop {
            let qmove = agent_a_move_recv.recv().unwrap();

            if let Ok(()) = Rb::validate_move(&game, qmove) {
                match Rb::apply_move(&mut game, qmove) {
                    MoveResult::Continue => {}
                    MoveResult::Draw => {
                        agent_a_event_send.send(GameEnd(None)).unwrap();
                        agent_b_event_send.send(GameEnd(None)).unwrap();
                        break;
                    }
                    MoveResult::Win(side) => {
                        agent_a_event_send.send(GameEnd(Some(side))).unwrap();
                        agent_b_event_send.send(GameEnd(Some(side))).unwrap();
                        break;
                    }
                }

                agent_a_event_send
                    .send(ValidMove(Clone::clone(&game)))
                    .unwrap();

                agent_b_event_send
                    .send(YourTurn(Clone::clone(&game)))
                    .unwrap();
            } else {
                agent_a_event_send.send(InvalidMove).unwrap();
                continue;
            }

            let qmove = agent_b_move_recv.recv().unwrap();

            if let Ok(()) = Rb::validate_move(&game, qmove) {
                match Rb::apply_move(&mut game, qmove) {
                    MoveResult::Continue => {}
                    MoveResult::Draw => {
                        agent_a_event_send.send(GameEnd(None)).unwrap();
                        agent_b_event_send.send(GameEnd(None)).unwrap();
                        break;
                    }
                    MoveResult::Win(side) => {
                        agent_a_event_send.send(GameEnd(Some(side))).unwrap();
                        agent_b_event_send.send(GameEnd(Some(side))).unwrap();
                        break;
                    }
                }

                agent_b_event_send
                    .send(ValidMove(Clone::clone(&game)))
                    .unwrap();

                agent_a_event_send
                    .send(YourTurn(Clone::clone(&game)))
                    .unwrap();
            } else {
                agent_b_event_send.send(InvalidMove).unwrap();
                continue;
            }
        }
    });

    (
        AgentCore {
            move_channel: agent_a_move_send,
            event_channel: agent_a_event_recv,
        },
        AgentCore {
            move_channel: agent_b_move_send,
            event_channel: agent_b_event_recv,
        },
    )
}
