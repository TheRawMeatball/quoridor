use crate::*;

pub fn quoridor_system(
    core: Res<AgentCore<FreeRulebook>>,
    mut game: ResMut<Game<FreeRulebook>>,
    side: Res<u8>,
    mut state: Local<MoveEventListenerState>,
    moves: Res<Events<MoveEvent>>,
) {
    if let Ok(event) = core.event_channel.try_recv() {
        match event {
            QuoridorEvent::MoveHappened(qmove) => {
                FreeRulebook::apply_move(&mut game, qmove);
            }
            //QuoridorEvent::InvalidMove => {}
            //QuoridorEvent::OpponentQuit => {}
            //QuoridorEvent::GameEnd(_) => {}
            _ => {}
        }
    }

    if let Some(qmove) = state.0.latest(&moves) {
        if *side == game.turn_of {
            core.move_channel.send(qmove.0).unwrap();
        }
    }
}
