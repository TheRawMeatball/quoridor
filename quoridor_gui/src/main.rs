use bevy::{prelude::*, winit::WinitConfig};
use quoridor_core::*;
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

    let mut thread = None;

    if args.contains(&String::from("--host")) {
        let (mut cores, t) = tbmp::new_game::<Quoridor>();
        thread = Some(t);
        core = cores.remove(0);
        tbmp::remote_agent::host(cores.remove(0), args[2].parse().unwrap());
    } else if args.contains(&String::from("--connect")) {
        core = tbmp::remote_agent::connect(args[2].parse().unwrap());
    } else {
        println!("Specify the --host <PORT> option or the --connect <IP:PORT> option");
        return;
    }

    let msg = core.event_channel.recv().unwrap();

    let (game, side) = match msg {
        GameEvent::GameStart(game, side) => (game, side),
        _ => unreachable!(),
    };

    let mut app = App::build();

    app.add_resource(WinitConfig {
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
    .add_plugin(GameSystemsPlugin);

    if let Some(mut t) = thread {
        app.add_system(
            (move || {
                t().ok();
            })
            .system(),
        );
    }

    app.run();
}
