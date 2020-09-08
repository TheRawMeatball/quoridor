pub mod free_rulebook;
pub mod standard_rulebook;
pub use free_rulebook::*;
pub use standard_rulebook::*;

#[macro_export]
macro_rules! generate_rulebook {
    ($($rulebook_ident:ident,)*) => {
        pub(crate) enum Quoridor {
            $(
                $rulebook_ident(QGame<$rulebook_ident>),
            )*
        }

        #[derive(Copy, Clone, Debug)]
        pub(crate) enum QGameType {
            $(
                $rulebook_ident,
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

        pub(crate) enum QSender {
            $(
                $rulebook_ident(Sender<<$rulebook_ident as Rulebook>::Move>),
            )*
        }

        impl QSender {
            pub(crate) fn send_move(&self, qmove: RulebookMove) -> Result<(), Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => c.send(match qmove {
                            RulebookMove::$rulebook_ident(qmv) => qmv,
                            _ => unreachable!(),
                        })?,
                    )*
                };
                Ok(())
            }
        }

        pub(crate) enum QReceiver {
            $(
                $rulebook_ident(Receiver<GameEvent<QGame<$rulebook_ident>>>),
            )*
        }

        impl QReceiver {
            pub(crate) fn recv_event(&self) -> Result<QGameEvent, Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => Ok(match c.try_recv()? {
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

            pub(crate) fn split(self) -> (QSender, QReceiver) {
                match self {
                    $(
                        Self::$rulebook_ident(c) => (QSender::$rulebook_ident(c.move_channel), QReceiver::$rulebook_ident(c.event_channel)),
                    )*
                }
            }

            pub(crate) fn connect(addr: std::net::SocketAddr, game_type: QGameType) -> (QAgent, Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>) {
                match game_type {
                    $(
                        QGameType::$rulebook_ident => {
                            let (c, t) = tbmp::remote_agent::connect(addr);
                            (QAgent::$rulebook_ident(c), Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>)
                        }
                    )*
                }
            }
        }

        pub(crate) trait AgentList {
            fn host(self, socket: u16, game_type: QGameType) -> Vec<Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>>;
        }

        impl AgentList for Vec<QAgent> {
            fn host(self, socket: u16, game_type: QGameType) ->  Vec<Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>> {
                match game_type {
                    $(
                        QGameType::$rulebook_ident => {
                            self
                                .into_iter()
                                .map(|core| {
                                    match core {
                                        QAgent::$rulebook_ident(c) => {
                                            Box::new(tbmp::remote_agent::host(vec![c], socket).remove(0)) as
                                                Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>
                                        },
                                        _ => unreachable!()
                                    }
                                })
                                .collect()
                        },
                    )*
                }
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

        impl QGameType {
            pub(crate) fn new_game(&self) -> (Vec<QAgent>, Box<dyn Send + Sync + FnMut() -> Result<MoveResult, Box<dyn Error>>>) {
                match self {
                    $(
                        Self::$rulebook_ident => {
                            let (cores, t) = tbmp::new_game::<QGame<$rulebook_ident>>();
                            (cores
                                .into_iter()
                                .map(|core| {
                                    QAgent::$rulebook_ident(core)
                                })
                                .collect(),
                                Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<MoveResult, Box<dyn Error>>>
                            )
                        },
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

    (
        [NO CONNECT]
        $(
            $rulebook_ident:ident,
        )*
    ) => {
        pub(crate) enum Quoridor {
            $(
                $rulebook_ident(QGame<$rulebook_ident>),
            )*
        }

        #[derive(Copy, Clone, Debug)]
        pub(crate) enum QGameType {
            $(
                $rulebook_ident,
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

        pub(crate) enum QSender {
            $(
                $rulebook_ident(Sender<<$rulebook_ident as Rulebook>::Move>),
            )*
        }

        impl QSender {
            pub(crate) fn send_move(&self, qmove: RulebookMove) -> Result<(), Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => c.send(match qmove {
                            RulebookMove::$rulebook_ident(qmv) => qmv,
                            _ => unreachable!(),
                        })?,
                    )*
                };
                Ok(())
            }
        }

        pub(crate) enum QReceiver {
            $(
                $rulebook_ident(Receiver<GameEvent<QGame<$rulebook_ident>>>),
            )*
        }

        impl QReceiver {
            pub(crate) fn recv_event(&self) -> Result<QGameEvent, Box<dyn Error>> {
                match self {
                    $(
                        Self::$rulebook_ident(c) => Ok(match c.try_recv()? {
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

            pub(crate) fn split(self) -> (QSender, QReceiver) {
                match self {
                    $(
                        Self::$rulebook_ident(c) => (QSender::$rulebook_ident(c.move_channel), QReceiver::$rulebook_ident(c.event_channel)),
                    )*
                }
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

        impl QGameType {
            pub(crate) fn new_game(&self) -> (Vec<QAgent>, Box<dyn Send + Sync + FnMut() -> Result<MoveResult, Box<dyn Error>>>) {
                match self {
                    $(
                        Self::$rulebook_ident => {
                            let (cores, t) = tbmp_core::new_game::<QGame<$rulebook_ident>>();
                            (cores
                                .into_iter()
                                .map(|core| {
                                    QAgent::$rulebook_ident(core)
                                })
                                .collect(),
                                Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<MoveResult, Box<dyn Error>>>
                            )
                        },
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
