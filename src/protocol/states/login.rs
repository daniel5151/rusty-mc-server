pub use crate::protocol::types::{Array, UuidString, VarInt};

pub mod server {
    use super::*;
    packets! {
        // 0x00 => Disconnect { reason: Chat }
        // 0x01 => EncryptionRequest {
        //     server_id: String,
        //     pubkey: Array<VarInt, u8>,
        //     verify_token: Array<VarInt, u8>
        // }
        0x02 => LoginSuccess {
            uuid: UuidString,
            username: String
        }
        0x03 => SetCompression { threshold: VarInt }
    }
}

pub mod client {
    use super::*;
    packets! {
        0x00 => LoginStart { name: String }
        // 0x01 => EncryptionResponse {
        //     shared_secret: Array<VarInt, u8>,
        //     verify_token: Array<VarInt, u8>
        // }
    }
}
