use super::types;

pub mod status;

#[derive(Debug)]
pub enum Error {
    BadLen(types::varint::Error),
    BadId(types::varint::Error),
    Io(std::io::Error),
    Status(status::Error),
}

#[derive(Debug)]
pub enum Packet {
    Status(status::Status),
}

impl Packet {
    pub fn write<T: std::io::Write>(self, outbuf: &mut T) -> Result<(), Error> {
        let mut buf: Vec<u8> = Vec::new();

        let len = match self {
            Packet::Status(status) => {
                let mut len = 0;
                len += types::VarInt::from(status.id())
                    .write(&mut buf)
                    .map_err(Error::BadId)?;
                len += status.write(&mut buf).map_err(Error::Status)?;
                len
            }
        };

        // write len
        types::VarInt::from(len as i32)
            .write(outbuf)
            .map_err(Error::BadLen)?;
        // then, write the rest of the packet
        outbuf.write(&buf).map_err(Error::Io)?;

        Ok(())
    }
}
