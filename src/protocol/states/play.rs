pub use crate::protocol::types::VarInt;

pub mod server {
    use super::*;
    packets! {
        // https://wiki.vg/Protocol#Join_Game
        0x25 => JoinGame {
            entity_id: i32,
            gamemode: u8,
            dimension: i32, // TODO?: make this an enum
            difficulty: u8,
            max_players: u8,
            level_type: String, // TODO: Make a strongly typed String Enum
            reduced_dbg_info: bool
        }
    }
}

pub mod client {
    use super::*;
    packets! {}
}
