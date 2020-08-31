use crate::*;

pub fn quoridor_system(
    core: Res<AgentCore<Quoridor>>,
    mut game: ResMut<Quoridor>,
    side: Res<u8>,
    mut state: Local<MoveEventListenerState>,
    moves: Res<Events<MoveEvent>>,
) {
    if let Ok(event) = core.event_channel.try_recv() {
        match event {
            GameEvent::MoveHappened(qmove) => {
                game.apply_move(qmove);
            }
            GameEvent::InvalidMove => {}
            //GameEvent::OpponentQuit => {}
            //GameEvent::GameEnd(_) => {}
            _ => {}
        }
    }

    if let Some(qmove) = state.0.latest(&moves) {
        if *side == game.turn_of {
            core.move_channel.send(qmove.0).unwrap();
        }
    }
}
