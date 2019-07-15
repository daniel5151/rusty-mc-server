use std::io;
use std::io::prelude::*;

use crate::protocol::WireProtocol;

#[derive(Debug, Clone, Copy)]
pub struct VarInt(i32);

impl WireProtocol for VarInt {
    fn proto_len(&self) -> usize {
        let mut val = self.0;
        let mut len = 0;
        loop {
            len += 1;
            val >>= 7;
            if val == 0 {
                return len;
            }
        }
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> std::io::Result<()> {
        let mut val = self.0;
        for _ in 0..5 {
            let mut tmp: u8 = (val & 0b01111111) as u8;
            val >>= 7;
            if val != 0 {
                tmp |= 0b10000000;
            }
            dst.write(&[tmp])?;
            if val == 0 {
                return Ok(());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "varint too long",
        ))
    }

    fn proto_decode<R: Read>(src: &mut R) -> std::io::Result<Self> {
        let mut result: i32 = 0;

        for n in 0..5 {
            let mut b = [0; 1];
            src.read(&mut b)?;

            let b = b[0];

            let val: i32 = (b & 0b01111111) as i32;
            result |= val << (7 * n);

            if (b & 0b10000000) == 0 {
                return Ok(result.into());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "varint too long",
        ))
    }
}

impl From<i32> for VarInt {
    fn from(x: i32) -> VarInt {
        VarInt(x)
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}
