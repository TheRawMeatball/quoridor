use crate::*;

pub fn quoridor_system(
    core: Res<AgentCore>,
    mut game: ResMut<Game>,
    side: Res<AgentSide>,
    mut state: Local<MoveEventListenerState>,
    moves: Res<Events<MoveEvent>>,
) {
    if let Ok(event) = core.event_channel.try_recv() {
        match event {
            QuoridorEvent::YourTurn(g) => {
                *game = g;
            }
            QuoridorEvent::ValidMove(g) => {
                *game = g;
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
