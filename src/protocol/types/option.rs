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

impl<T: WireProtocol> WireProtocol for Option<T> {
    fn proto_len(&self) -> usize {
        match self {
            Some(inner) => 1 + T::proto_len(inner),
            None => 1,
        }
    }

    fn proto_encode<W: Write>(&self, dst: &mut W) -> io::Result<()> {
        match self {
            Some(inner) => {
                bool::proto_encode(&true, dst)?;
                T::proto_encode(inner, dst)?;
            }
            None => {
                bool::proto_encode(&false, dst)?;
            }
        }
        Ok(())
    }

    fn proto_decode<R: Read>(src: &mut R) -> io::Result<Option<T>> {
        if bool::proto_decode(src)? {
            Ok(Some(T::proto_decode(src)?))
        } else {
            Ok(None)
        }
    }
}
