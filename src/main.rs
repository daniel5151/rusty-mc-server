use std::net::{TcpListener, TcpStream};

#[macro_use]
mod util;

mod play_stub;
mod setup;

pub mod protocol;

use crate::protocol::PacketRead;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadClientPacket(std::io::Error),
    BadServerPacket(std::io::Error),
    BadSequence,
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Got a connection!");

    use protocol::handshake::{NextState, Packet};

    let packet = Packet::decode(&mut stream).map_err(Error::BadClientPacket)?;
    println!("Got handshake packet: {:#?}", packet);
    let handshake = match packet {
        Packet::Handshake(handshake) => handshake,
    };

    // TODO: use some of the other handshake params?

    match handshake.next_state {
        NextState::Status => setup::slp::handle_slp(stream),
        NextState::Login => {
            setup::login::handle_login(&mut stream)?;
            // load player into world
            play_stub::handle_play(stream)
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:25565")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        if let Err(e) = handle_client(stream?) {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
