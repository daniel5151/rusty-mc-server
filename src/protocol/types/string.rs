use std::io;
use std::io::prelude::*;

use crate::protocol::WireProtocol;

use super::VarInt;

impl WireProtocol for String {
    fn proto_len(&self) -> usize {
        VarInt::from(self.len() as i32).proto_len() + self.len()
    }

    fn proto_encode(&self, dst: &mut Write) -> io::Result<()> {
        VarInt::from(self.len() as i32).proto_encode(dst)?;
        dst.write(self.as_ref())?;
        Ok(())
    }

    fn proto_decode(src: &mut Read) -> io::Result<Self> {
        let len: i32 = VarInt::proto_decode(src)?.into();

        let mut str_bytes: Vec<u8> = vec![0; len as usize];
        src.read_exact(&mut str_bytes)?;

        let string = std::str::from_utf8(&str_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "malformed utf8 string"))?;

        Ok(string.to_string())
    }
}
