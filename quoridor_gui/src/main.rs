use bevy::{prelude::*, winit::WinitConfig};
#[allow(unused_imports)]
use quoridor_core::{*, 
    standard_rulebook::StandardQuoridor,
    free_rulebook::FreeQuoridor,
};
use tbmp::*;

pub type Quoridor = StandardQuoridor;

mod components;
mod constants;
mod systems;
pub(crate) use components::*;
pub(crate) use constants::*;
use systems::*;
use std::error::Error;

pub struct MoveEvent(Move);
#[derive(Default)]
pub struct MoveEventListenerState(EventReader<MoveEvent>);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let core;

    let mut threads = vec![];

    if args.contains(&String::from("--host")) {
        let (mut cores, t) = tbmp::new_game::<Quoridor>();
        threads.push(Box::new(t) as Box<dyn Send + Sync + FnMut() -> Result<(), Box<dyn Error>>>);
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
    } else if args.contains(&String::from("--headless")) {
        let (cores, mut game_thread) = tbmp::new_game::<Quoridor>();
        let mut player_threads = tbmp::remote_agent::host(cores, args[2].parse().unwrap());
        loop {
            game_thread().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
            for t in player_threads.iter_mut() {
                t().unwrap();
            }
        }
    } else {
        println!(r"Usage: --host <PORT>
                          --headless <PORT>
                          --connect <IP:PORT>");
        return;
    }

    let msg = loop {
        threads[0]().ok();
        if let Ok(msg) = core.event_channel.try_recv()
        {
            break msg;
        }
    };

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
