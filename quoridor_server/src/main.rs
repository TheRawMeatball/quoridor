#[allow(unused_imports)]
use quoridor_core::{rulebooks::*, QGame};
use std::env;
use tbmp::*;

type Quoridor = QGame<FreeQuoridor>;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    loop {
        println!("NEW GAME");
        let (cores, mut game_thread) = tbmp::new_game::<Quoridor>();
        let mut player_threads = tbmp::remote_agent::host(cores, args[1].parse().unwrap());
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
