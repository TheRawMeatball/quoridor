use bevy::{prelude::*, winit::WinitConfig};
use quoridor_core::*;
use tbmp_core::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod components;
mod constants;
mod systems;
pub use components::*;
pub use constants::*;
use systems::*;

pub struct MoveEvent(Move);
#[derive(Default)]
pub struct MoveEventListenerState(EventReader<MoveEvent>);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let core;

    if args.contains(&String::from("--host")) {
        let mut cores = tbmp_core::new_game::<Quoridor>();
        core = cores.remove(0);
        tbmp_remote_agent::host(cores.remove(0), PORT);
    } else if args.contains(&String::from("--connect")) {
        core =
            tbmp_remote_agent::connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT));
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
