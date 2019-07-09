use super::types;
use crate::Mode;

pub mod handshake;
pub mod status;

#[derive(Debug)]
pub enum Error {
    BadLen(types::varint::Error),
    BadId(types::varint::Error),
    Handshake(handshake::Error),
    Status(status::Error),
}

#[derive(Debug)]
pub enum Packet {
    Handshake(handshake::Handshake),
    Status(status::Status),
}

impl Packet {
    pub fn unwrap_handshake(self) -> Option<handshake::Handshake> {
        if let Packet::Handshake(handshake) = self {
            Some(handshake)
        } else {
            None
        }
    }

    pub fn unwrap_status(self) -> Option<status::Status> {
        if let Packet::Status(status) = self {
            Some(status)
        } else {
            None
        }
    }
}

pub fn read<T: std::io::Read>(mode: Mode, inbuf: &mut T) -> Result<Packet, Error> {
    let len = types::VarInt::read(inbuf).map_err(Error::BadLen)?;
    let id = types::VarInt::read(inbuf).map_err(Error::BadId)?;

    // TODO: actually do something with len?
    let _ = len;

    let data = match mode {
        Mode::Handshake => {
            Packet::Handshake(handshake::Handshake::read(inbuf).map_err(Error::Handshake)?)
        }
        Mode::Status => {
            Packet::Status(status::Status::read(id.into(), inbuf).map_err(Error::Status)?)
        }
        _ => unimplemented!(),
    };

    Ok(data)
}
