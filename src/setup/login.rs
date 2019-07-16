use std::net::TcpStream;

use crate::protocol::{
    self,
    types::{UuidString, VarInt},
};
use crate::util::*;
use crate::Error;

pub fn handle_login(mut stream: TcpStream) -> Result<(), Error> {
    use protocol::login::client;
    use protocol::login::server;

    let client::packets::LoginStart { name: username } =
        expect_pkt!(&mut stream, login, LoginStart);

    // Skip setting up encryption (for now?)
    // Disable compression (for now)

    send_pkt(
        &mut stream,
        server::packets::SetCompression {
            threshold: VarInt::from(-1),
        },
    )?;

    // Send off LoginSuccess
    // TODO: actually associate username with UUID instead of just generating a
    // new one each time
    send_pkt(
        &mut stream,
        server::packets::LoginSuccess {
            uuid: UuidString::from(uuid::Uuid::new_v4()),
            username,
        },
    )?;

    Ok(())
}
