//! Minecraft protocol length-prefixed array data type

use std::io;
use std::io::prelude::*;
use std::marker::PhantomData;

use crate::protocol::WireProtocol;

use super::VarInt;

#[derive(Debug)]
pub struct Array<L, T>(Vec<T>, PhantomData<L>)
where
    L: WireProtocol,
    T: WireProtocol;

impl<L, T> WireProtocol for Array<L, T>
where
    L: WireProtocol,
    T: WireProtocol,
{
    fn proto_len(&self) -> usize {
        let mut len = 0;

        len += VarInt::from(self.0.len() as i32).proto_len();
        len += self.0.iter().map(|v| v.proto_len()).sum::<usize>();

        len
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        VarInt::from(self.0.len() as i32).proto_encode(dst)?;
        for v in self.0.iter() {
            v.proto_encode(dst)?;
        }
        Ok(())
    }

    fn proto_decode<R: Read>(src: &mut R) -> io::Result<Array<L, T>> {
        let len: i32 = VarInt::proto_decode(src)?.into();
        let mut result: Array<L, T> = Vec::new().into();
        for _ in 0..=len {
            result.0.push(T::proto_decode(src)?);
        }
        Ok(result)
    }
}

impl<L, T> From<Vec<T>> for Array<L, T>
where
    L: WireProtocol,
    T: WireProtocol,
{
    fn from(x: Vec<T>) -> Array<L, T> {
        Array(x, PhantomData)
    }
}

impl<L, T> Into<Vec<T>> for Array<L, T>
where
    L: WireProtocol,
    T: WireProtocol,
{
    fn into(self) -> Vec<T> {
        self.0
    }
}
