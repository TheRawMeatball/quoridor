use bevy::prelude::*;

mod board_systems_mod;
mod quoridor_system_mod;
mod setup_mod;

use board_systems_mod::*;
use quoridor_system_mod::*;
use setup_mod::*;

pub struct GameSystemsPlugin;

#[derive(Debug)]
pub struct BoardState {
    highlight_pawn: bool,
    can_highlight: bool,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            highlight_pawn: false,
            can_highlight: true,
        }
    }
}

pub struct ExitTimer {
    timer: Timer,
    enabled: bool,
}

impl Default for ExitTimer {
    fn default() -> Self {
        ExitTimer {
            timer: Timer::from_seconds(2.0, false),
            enabled: false,
        }
    }
}

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_system(setup.system())
            .init_resource::<BoardState>()
            .init_resource::<ExitTimer>()
            .add_stage_before(stage::UPDATE, "first_pass")
            .add_system_to_stage("first_pass", board_update_system.system())
            .add_system(input_system.system())
            .add_system(quoridor_system.system());
    }
}
