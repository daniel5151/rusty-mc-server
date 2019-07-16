pub use crate::protocol::types::VarInt;

pub mod server {
    use super::*;
    packets! {
        // https://wiki.vg/Protocol#Join_Game
        0x25 => JoinGame {
            entity_id: i32,
            gamemode: u8,
            dimension: i32, // TODO?: make this an enum
            max_players: u8,
            level_type: String, // TODO: Make a strongly typed String Enum
            view_distance: VarInt,
            reduced_dbg_info: bool
        }
    }
}

pub mod client {
    use super::*;
    packets! {
        0x05 => ClientSettings {
            locale: String, // e.g: en_GB
            view_distance: i8,
            chat_mode: VarInt, // TODO?: make this a proper enum
            chat_colors: bool,
            skin_parts: u8,
            main_hand: VarInt // TODO: make this a proper enum
        }
    }
}
