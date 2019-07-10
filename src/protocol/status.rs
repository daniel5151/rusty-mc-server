pub use std::io;
pub use std::io::prelude::*;

pub use super::types::VarInt;
pub use super::{PacketRead, PacketWrite, WireProtocol};

pub mod server {
    use super::*;
    packets! {
        0x00 => Response {
            // TODO: should be something more robust than just a string JSON
            json: String
        }
        0x01 => Pong { payload: i64 }
    }
}
pub mod client {
    use super::*;
    packets! {
        0x00 => Request {}
        0x01 => Ping { payload: i64 }
    }
}
