use std::io;
use std::io::prelude::*;

use uuid::Uuid;

use crate::protocol::WireProtocol;

impl WireProtocol for Uuid {
    fn proto_len(&self) -> usize {
        // 2 64bit numbers
        16
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        dst.write(self.as_bytes())?;
        Ok(())
    }

    fn proto_decode<R: Read>(src: &mut R) -> io::Result<Self> {
        let mut bytes = [0; 16];
        src.read_exact(&mut bytes)?;

        let uuid = Uuid::from_bytes(bytes);

        Ok(uuid)
    }
}

#[derive(Debug)]
pub struct UuidString(Uuid);

impl WireProtocol for UuidString {
    fn proto_len(&self) -> usize {
        // TODO: This could be hardcoded, since the strings should have constant length
        self.0.to_string().len()
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> std::io::Result<()> {
        dst.write(self.0.to_string().as_bytes())?;
        Ok(())
    }

    fn proto_decode<R: Read>(src: &mut R) -> std::io::Result<Self> {
        let _ = src;
        unimplemented!()
    }
}

impl From<Uuid> for UuidString {
    fn from(x: Uuid) -> UuidString {
        UuidString(x)
    }
}

impl Into<Uuid> for UuidString {
    fn into(self) -> Uuid {
        self.0
    }
}
