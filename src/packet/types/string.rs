use super::VarInt;

use std::string::String as StdString;

#[derive(Debug, Clone)]
pub struct String(StdString);

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    NotUtf8(std::str::Utf8Error),
    BadLen(super::varint::Error),
}

impl String {
    // TODO: validate read length
    pub fn read<T: std::io::Read>(buf: &mut T) -> Result<String, Error> {
        let len: i32 = VarInt::read(buf).map_err(Error::BadLen)?.into();

        // TODO: do some bounds checking on the usize conversion
        let mut str_bytes: Vec<u8> = vec![0; len as usize];
        buf.read_exact(&mut str_bytes).map_err(Error::Io)?;

        let string = std::str::from_utf8(&str_bytes).map_err(Error::NotUtf8)?;

        Ok(String(string.to_string()))
    }

    // TODO: validate write length
    pub fn write<T: std::io::Write>(self, buf: &mut T) -> Result<usize, Error> {
        let mut n_written = 0;
        n_written += VarInt::from(self.0.len() as i32)
            .write(buf)
            .map_err(Error::BadLen)?;
        n_written += buf.write(self.0.as_ref()).map_err(Error::Io)?;
        Ok(n_written)
    }
}

impl From<StdString> for String {
    fn from(x: StdString) -> String {
        String(x)
    }
}

impl Into<StdString> for String {
    fn into(self) -> StdString {
        self.0
    }
}
