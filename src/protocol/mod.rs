//! MC Protocol and Packets

pub mod types;

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

use types::VarInt;

/// A trait used for data which can be encoded/decoded as is.
pub trait WireProtocol: Sized {
    fn proto_len(&self) -> usize;
    fn proto_encode(&self, dst: &mut Write) -> io::Result<()>;
    fn proto_decode(src: &mut Read) -> io::Result<Self>;
}

/// A trait for encoding the body of a single packet type.
pub trait PacketWrite {
    fn inner_len(&self) -> usize;
    fn inner_encode(&self, dst: &mut Write) -> io::Result<()>;

    /// Writes a full packet to a writer, including length.
    ///
    /// **TODO:** add support for compression.
    fn write(&self, dst: &mut Write) -> io::Result<()> {
        let len = self.inner_len();
        VarInt::proto_encode(&((len as i32).into()), dst)?;
        self.inner_encode(dst)
    }
}

/// A trait for decoding any of the packet types in one ID namespace.
pub trait PacketRead: Sized {
    fn inner_decode(src: &mut Read) -> io::Result<Self>;

    /// Reads a new packet from a reader, including length.
    ///
    /// **TODO:** add support for compression.
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        let proto_len = VarInt::proto_decode(src)?;
        Self::inner_decode(&mut src.take(Into::<i32>::into(proto_len) as u64))
    }
}

macro_rules! packets {
    ($($id:expr => $name:ident { $($packet:tt)* })*) => {
        $(proto_struct!{ $name { $($packet)* } })*

        #[derive(Debug)]
        pub enum Packet {
            $($name($name)),*
        }

        impl PacketRead for Packet {
            fn inner_decode(src: &mut Read) -> io::Result<Self> {
                match VarInt::proto_decode(src)?.into() {
                    $(
                        $id => $name::proto_decode(src).map(Packet::$name),
                    )*
                    _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "unknown packet id"))
                }
            }
        }

        $(impl PacketWrite for $name {
            fn inner_len(&self) -> usize {
                let id_len = VarInt::proto_len(&$id.into());
                id_len + <Self as WireProtocol>::proto_len(self)
            }

            fn inner_encode(&self, dst: &mut Write) -> io::Result<()> {
                VarInt::proto_encode(&$id.into(), dst)?;
                <Self as WireProtocol>::proto_encode(self, dst)
            }
        })*
    }
}

macro_rules! proto_struct {
    // Regular structs.
    ($name:ident { $($fname:ident: $fty:ty),+ }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname: $fty),*
        }

        impl WireProtocol for $name {
            fn proto_len(&self) -> usize {
                0 $(+ <$fty as WireProtocol>::proto_len(&self.$fname))*
            }

            fn proto_encode(&self, dst: &mut Write) -> io::Result<()> {
                $(<$fty as WireProtocol>::proto_encode(&self.$fname, dst)?;)*
                Ok(())
            }

            fn proto_decode(src: &mut Read) -> io::Result<$name> {
                Ok($name {
                    $($fname: <$fty as WireProtocol>::proto_decode(src)?),*
                })
            }
        }
    };
    // No field structs (unit values).
    ($name:ident {}) => {
        #[derive(Debug)]
        pub struct $name;

        impl WireProtocol for $name {
            fn proto_len(&self) -> usize { 0 }

            fn proto_encode(&self, _: &mut Write) -> io::Result<()> {
                Ok(())
            }

            fn proto_decode(_: &mut Read) -> io::Result<$name> {
                Ok($name)
            }
        }
    };
    // Custom encode/decode structs.
    ($name:ident { $($fname:ident: $fty:ty),+; $impl_struct:item }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fname: $fty),*
        }

        $impl_struct
    }
}

// must be at the bottom, since they use macros above
pub mod handshake;
pub mod status;
