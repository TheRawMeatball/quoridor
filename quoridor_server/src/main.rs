use bimap::BiMap;
use crossbeam_channel::{Receiver, Sender};
#[allow(unused_imports)]
use quoridor_core::{rulebooks::*, *};
use std::env;
use std::error::Error;
use tbmp::*;

generate_rulebook! {
    FreeQuoridor,
    StandardQuoridor,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let game_type = QGameType::StandardQuoridor;

    loop {
        println!("NEW GAME");
        let (cores, mut game_thread) = game_type.new_game();
        let mut player_threads = cores.host(args[1].parse().unwrap(), game_type);
        loop {
            let x = game_thread();
            for t in player_threads.iter_mut() {
                t().ok();
            }
            match x {
                Ok(MoveResult::Continue) => {}
                _ => break,
            }
        }
    }
}
