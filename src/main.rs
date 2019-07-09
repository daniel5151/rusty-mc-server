use std::net::{TcpListener, TcpStream};

pub mod packet;

pub enum Mode {
    Handshake,
    Status,
    Login,
    Play,
}

use serde_json::json;

const PROTOCOL: u32 = 490;
const NAME: &str = "1.14.3";
const MOTD: &str = "Hello from Rust!";

#[derive(Debug)]
pub enum Error {
    BadClientPacket(packet::client::Error),
    BadServerPacket(packet::server::Error),
    BadSequence,
    Other(String),
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut mode = Mode::Handshake;

    println!("Got a client!");

    let packet = packet::client::read(mode, &mut stream).map_err(Error::BadClientPacket)?;
    println!("Got a packet: {:#?}", packet);

    let handshake = packet.unwrap_handshake().unwrap();
    mode = match handshake.next_state {
        1 => Mode::Status,
        2 => Mode::Login,
        n => {
            return Err(Error::Other(format!(
                "Unexpected next_state in handshake: {}",
                n
            )))
        }
    };

    use packet::server::Packet as PacketS;

    match mode {
        Mode::Status => {
            use packet::client::read;
            use packet::client::status::Status as StatusC;
            use packet::server::status::Status as StatusS;

            // Expect a empty Request packet
            let pkt = read(mode, &mut stream).map_err(Error::BadClientPacket)?;
            println!("Got another packet: {:#?}", pkt);
            let status = pkt.unwrap_status().unwrap();

            if let StatusC::Request = status {
            } else {
                return Err(Error::BadSequence);
            }

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
                }
            });
            let pkt = PacketS::Status(StatusS::Response {
                json: slp.to_string(),
            });
            println!("Sending a packet: {:#?}", pkt);
            pkt.write(&mut stream).map_err(Error::BadServerPacket)?;

            // Expect a ping packet
            let pkt = read(Mode::Status, &mut stream).map_err(Error::BadClientPacket)?;
            println!("Got another packet: {:#?}", pkt);
            let status = pkt.unwrap_status().unwrap();

            if let StatusC::Ping { payload } = status {
                // send pong back with same payload
                let pkt = PacketS::Status(StatusS::Pong { payload });
                println!("Sending a packet: {:#?}", pkt);
                pkt.write(&mut stream).map_err(Error::BadServerPacket)?;
            } else {
                return Err(Error::BadSequence);
            }
        }
        Mode::Login => {
            eprintln!("Login not working yet!");
            unimplemented!()
        }
        // Guaranteed unreachable, since the next_state match arm returns an
        // error if the next mode is anything but Login or Status
        _ => unreachable!(),
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:25565")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        if let Err(e) = handle_client(stream?) {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
