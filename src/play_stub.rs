//! TODO: actually refactor this into something presentable

use std::net::TcpStream;

use crate::protocol::{self, types::VarInt};
use crate::util::*;
use crate::Error;

pub fn handle_play(mut stream: TcpStream) -> Result<(), Error> {
    use protocol::play::client;
    use protocol::play::server;

    send_pkt(
        &mut stream,
        server::packets::JoinGame {
            entity_id: 0,
            gamemode: 0b0010, // Creative mode, not hardcore
            dimension: 0,     // Overworld
            max_players: 100, // TODO: remove hardcode
            level_type: "default".to_string(),
            view_distance: VarInt::from(32),
            reduced_dbg_info: false,
        },
    )?;

    let _client_settings = expect_pkt!(&mut stream, play, ClientSettings);

    // And now, I gotta send Chunk data.
    // Ahhhhh

    Ok(())
}
