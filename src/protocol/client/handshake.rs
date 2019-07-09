use super::types;

#[derive(Debug)]
pub enum Error {
    BadProtocol(types::varint::Error),
    BadAddress(types::string::Error),
    BadPort(std::io::Error),
    BadNextState(types::varint::Error),
}

#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub address: String,
    pub port: u16,
    pub next_state: i32,
}

impl Handshake {
    pub fn read<T: std::io::Read>(inbuf: &mut T) -> Result<Handshake, Error> {
        let protocol_version = types::VarInt::read(inbuf).map_err(Error::BadProtocol)?;
        let address = types::String::read(inbuf).map_err(Error::BadAddress)?;
        let port = {
            let mut port_buf = [0; 2];
            inbuf.read(&mut port_buf).map_err(Error::BadPort)?;
            u16::from_le_bytes(port_buf)
        };
        let next_state = types::VarInt::read(inbuf).map_err(Error::BadNextState)?;

        Ok(Handshake {
            protocol_version: protocol_version.into(),
            address: address.into(),
            port: port,
            next_state: next_state.into(),
        })
    }
}
