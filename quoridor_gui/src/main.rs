use bevy::{prelude::*, winit::WinitConfig};
#[allow(unused_imports)]
use quoridor_core::{free_rulebook::FreeQuoridor, standard_rulebook::StandardQuoridor, *};
use tbmp::*;

pub type Quoridor = StandardQuoridor;

mod components;
mod constants;
mod systems;
pub(crate) use components::*;
pub(crate) use constants::*;
use std::error::Error;
use systems::*;

pub struct MoveEvent(Move);
#[derive(Default)]
pub struct MoveEventListenerState(EventReader<MoveEvent>);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let core;

    let mut threads = vec![];

    if args.contains(&String::from("--host")) {
        let (mut cores, mut t) = tbmp::new_game::<Quoridor>();
        threads.push(Box::new(move || match t() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
            as Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>);
        core = cores.remove(0);
        let mut tb = tbmp::remote_agent::host(vec![cores.remove(0)], args[2].parse().unwrap());
        let t = move || -> Result<(), Box<dyn Error>> {
            for t in tb.iter_mut() {
                t()?;
            }
            Ok(())
        };
        threads.push(Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>);
    } else if args.contains(&String::from("--connect")) {
        let (c, t) = tbmp::remote_agent::connect(args[2].parse().unwrap());
        core = c;
        threads.push(Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>);
    } else {
        println!(r"Usage: --host <PORT> / --connect <IP:PORT>");
        return;
    }

    let msg = loop {
        threads[0]().ok();
        if let Ok(msg) = core.event_channel.try_recv() {
            break msg;
        }
    };

    println!("{:?}", msg);

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

    for mut t in threads.into_iter() {
        let thread = move || {
            t().ok();
        };
        app.add_system(thread.system());
    }

    app.run();
}
