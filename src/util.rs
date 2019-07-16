use crate::protocol::PacketWrite;
use crate::Error;

// TODO: toggle packet logging

/// Extract specified client packet from src, returning a [Error::BadSequence]
/// if unexpected packet was extracted in it's place
macro_rules! expect_pkt {
    ($src:expr, $state:ident, $name:ident) => {{
        use crate::protocol::$state::client;
        use crate::protocol::PacketRead;

        #[allow(unreachable_patterns)]
        match client::Packet::decode($src).map_err(Error::BadClientPacket)? {
            client::Packet::$name(pkt) => {
                println!("Got a packet: {:#?}", pkt);
                pkt
            }
            _ => return Err(Error::BadSequence),
        }
    }};
}

/// Send a packet to the specified dst, consuming it in the process.
pub fn send_pkt<W: std::io::Write, T: PacketWrite + std::fmt::Debug>(
    dst: &mut W,
    pkt: T,
) -> Result<(), Error> {
    println!("Sending a packet: {:#?}", pkt);
    pkt.encode(dst).map_err(Error::BadServerPacket)
}
