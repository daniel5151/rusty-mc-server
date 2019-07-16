pub use crate::protocol::types::VarInt;

packets! {
    0x00 => Handshake {
        proto_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: NextState
    }
}

#[derive(Debug)]
pub enum NextState {
    Status,
    Login,
}

impl WireProtocol for NextState {
    fn proto_len(&self) -> usize {
        1
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        let i = match self {
            NextState::Status => 1,
            NextState::Login => 2,
        };
        VarInt::proto_encode(&i.into(), dst)
    }

    fn proto_decode<R: Read>(src: &mut R) -> io::Result<Self> {
        match VarInt::proto_decode(src)?.into() {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid next_state",
            )),
        }
    }
}
