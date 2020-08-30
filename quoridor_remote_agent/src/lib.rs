use quoridor_core::*;
use std::net::*;

pub fn connect<Rb: Rulebook>(addr: SocketAddr) -> AgentCore<Rb> {
    let (tx, rx) = remote_channel::connect(addr).unwrap();
    AgentCore {
        move_channel: tx,
        event_channel: rx,
    }
}

pub fn host<Rb: Rulebook>(core: AgentCore<Rb>, socket: u16) {
    remote_channel::offer_connection(core.move_channel, core.event_channel, socket).unwrap();
}
