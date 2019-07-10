// The MIT License (MIT)
//
// Copyright (c) 2015 PistonDevelopers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Modified and Modernized by Daniel Prilik 2019

use std::io;
use std::io::prelude::*;

use crate::protocol::WireProtocol;

macro_rules! impl_WireProtocol {
    ($type:ty) => {
        impl WireProtocol for $type {
            fn proto_len(&self) -> usize {
                std::mem::size_of::<$type>()
            }

            fn proto_encode(&self, dst: &mut Write) -> io::Result<()> {
                dst.write(&<$type>::to_le_bytes(*self))?;
                Ok(())
            }

            fn proto_decode(src: &mut Read) -> io::Result<$type> {
                let mut buf = <$type>::to_le_bytes(0);
                src.read(&mut buf)?;
                Ok(<$type>::from_le_bytes(buf))
            }
        }
    };
}

impl_WireProtocol!(i8);
impl_WireProtocol!(i16);
impl_WireProtocol!(i32);
impl_WireProtocol!(i64);

impl_WireProtocol!(u8);
impl_WireProtocol!(u16);
impl_WireProtocol!(u32);
impl_WireProtocol!(u64);

macro_rules! impl_WireProtocolFloat {
    ($type:ty, $equiv:ty) => {
        impl WireProtocol for $type {
            fn proto_len(&self) -> usize {
                std::mem::size_of::<$type>()
            }

            fn proto_encode(&self, dst: &mut Write) -> io::Result<()> {
                <$equiv>::proto_encode(&self.to_bits(), dst)
            }

            fn proto_decode(src: &mut Read) -> io::Result<$type> {
                Ok(<$type>::from_bits(<$equiv>::proto_decode(src)?))
            }
        }
    };
}

impl_WireProtocolFloat!(f32, u32);
impl_WireProtocolFloat!(f64, u64);

impl WireProtocol for bool {
    fn proto_len(&self) -> usize {
        1
    }

    fn proto_encode(&self, dst: &mut Write) -> io::Result<()> {
        dst.write(if *self { &[1] } else { &[0] })?;
        Ok(())
    }

    fn proto_decode(src: &mut Read) -> io::Result<bool> {
        let mut buf = [0; 1];
        src.read(&mut buf)?;
        let value = buf[0];

        if value > 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                &format!("Invalid bool value, expecting 0 or 1, got {}", value)[..],
            ))
        } else {
            Ok(value == 1)
        }
    }
}
