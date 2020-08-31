use bevy::{prelude::*, winit::WinitConfig};
use quoridor_core::*;
use std::net::SocketAddr;
use tbmp::*;

mod components;
mod constants;
mod systems;
pub(crate) use components::*;
pub(crate) use constants::*;
use systems::*;

pub struct MoveEvent(Move);
#[derive(Default)]
pub struct MoveEventListenerState(EventReader<MoveEvent>);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let core;

    if args.contains(&String::from("--host")) {
        let mut cores = tbmp::new_game::<Quoridor>();
        core = cores.remove(0);
        tbmp::remote_agent::host(cores.remove(0), PORT);
    } else if args.contains(&String::from("--connect")) {
        core = tbmp::remote_agent::connect(SocketAddr::new(args[2].parse().unwrap(), PORT));
    } else {
        println!("Specify desired outcome");
        return;
    }

    let msg = core.event_channel.recv().unwrap();

    let (game, side) = match msg {
        GameEvent::GameStart(game, side) => (game, side),
        _ => unreachable!(),
    };

    App::build()
        .add_resource(WinitConfig {
            return_from_run: true,
        })
        .add_resource(WindowDescriptor {
            width: 720,
            height: 720,
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(core)
        .add_resource(game)
        .add_resource(side)
        .add_event::<MoveEvent>()
        //// Adds frame time diagnostics
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        //// Adds a system that prints diagnostics to the console
        //.add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .add_plugin(GameComponentsPlugin)
        .add_plugin(GameSystemsPlugin)
        .run();
}
