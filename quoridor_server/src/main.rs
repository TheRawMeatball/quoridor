use quoridor_core::*;
use tbmp::*;
use standard_rulebook::StandardQuoridor;
use std::env;

type Quoridor = StandardQuoridor;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    
    loop {
        let (cores, mut game_thread) = tbmp::new_game::<Quoridor>();
        let mut player_threads = tbmp::remote_agent::host(cores, args[2].parse().unwrap());
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
