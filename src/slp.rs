use std::io::prelude::*;
use std::net::TcpStream;

use serde_json::json;

use crate::protocol::{self, PacketRead, PacketWrite};
use crate::Error;

// TODO: don't hardcode these!
const PROTOCOL: u32 = 490;
const NAME: &str = "1.14.3";
const MOTD: &str = "Hello from Rust!";

pub fn handle_slp(mut stream: TcpStream) -> Result<(), Error> {
    use protocol::status::client;
    use protocol::status::server;

    // Expect a empty Request packet
    let packet = client::Packet::read(&mut stream).map_err(Error::BadClientPacket)?;
    println!("Got a status packet: {:#?}", packet);
    match packet {
        client::Packet::Request(..) => (),
        _ => return Err(Error::BadSequence),
    }

    // construct server info

    // load favicon, and convert it to base64
    let favicon = &std::fs::File::open("assets/favicon.png")
        .map_err(Error::Io)?
        .bytes()
        .collect::<Result<Vec<u8>, _>>()
        .map_err(Error::Io)?;

    // Send over server info
    let slp = json!({
        "version": {
            "name": NAME,
            "protocol": PROTOCOL
        },
        "players": {
            "max": 100,
            "online": 2,
            "sample": [
                {
                    "name": "not",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                },
                {
                    "name": "real",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d21"
                },
                {
                    "name": "players",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d22"
                }
            ]
        },
        "description": {
            "text": MOTD
        },
        "favicon": format!("data:image/png;base64,{}", base64::encode(favicon))
    });

    // Send off server info
    let response = server::Response {
        json: slp.to_string(),
    };
    println!("Sending a packet: {:#?}", response);
    response
        .write(&mut stream)
        .map_err(Error::BadServerPacket)?;

    // Expect a ping packet
    let packet = client::Packet::read(&mut stream).map_err(Error::BadClientPacket)?;
    println!("Got a status packet: {:#?}", packet);
    let ping = match packet {
        client::Packet::Ping(ping) => ping,
        _ => return Err(Error::BadSequence),
    };

    // send pong back with same payload
    let pong = server::Pong {
        payload: ping.payload,
    };
    println!("Sending a packet: {:#?}", pong);
    pong.write(&mut stream).map_err(Error::BadServerPacket)?;

    Ok(())
}
