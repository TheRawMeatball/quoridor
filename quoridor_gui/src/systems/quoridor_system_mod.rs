use crate::*;
use bevy::app::AppExit;

pub(crate) fn quoridor_system(
    time: Res<Time>,
    mut exit_event: ResMut<Events<AppExit>>,
    mut exit_timer: ResMut<ExitTimer>,
    core: Res<QAgent>,
    mut game: ResMut<Quoridor>,
    side: Res<u8>,
    mut state: Local<MoveEventListenerState>,
    moves: Res<Events<MoveEvent>>,
) {
    if let Ok(event) = core.recv_event() {
        match event {
            QGameEvent::MoveHappened(qmove) => {
                game.apply_move(&qmove);
            }
            QGameEvent::InvalidMove => println!("Invalid move!"),
            QGameEvent::GameEnd(side) => {
                println!("Player {} wins!", side.unwrap() + 1);
                exit_timer.enabled = true;
            }
            //GameEvent::OpponentQuit => {}
            _ => {}
        }
    }

    if let Some(qmove) = state.0.latest(&moves) {
        if *side == game.turn_of() {
            core.send_move(RulebookMove::wrap(&game, &qmove.0)).unwrap();
        }
    }

    if exit_timer.enabled {
        exit_timer.timer.tick(time.delta_seconds);
        if exit_timer.timer.finished {
            exit_event.send(AppExit);
        }
    }
}
