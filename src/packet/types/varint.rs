#[derive(Debug, Clone, Copy)]
pub struct VarInt(i32);

#[derive(Debug)]
pub enum Error {
    BufferTooSmall,
    TooLong,
    Io(std::io::Error),
}

impl VarInt {
    pub fn read<T: std::io::Read>(buf: &mut T) -> Result<VarInt, Error> {
        let mut num_read = 0;
        let mut result: i32 = 0;

        loop {
            let mut b = [0; 1];
            let n = buf.read(&mut b).map_err(|e| Error::Io(e))?;
            if n == 0 {
                return Err(Error::BufferTooSmall);
            }

            let b = b[0];

            let val: i32 = (b & 0b01111111) as i32;
            result |= val << (7 * num_read);
            num_read += 1;
            if num_read > 5 {
                return Err(Error::TooLong);
            }

            if (b & 0b10000000) == 0 {
                return Ok(result.into());
            }
        }
    }

    pub fn write<T: std::io::Write>(self, buf: &mut T) -> Result<usize, Error> {
        let mut n_written = 0;
        let mut val = self.0;
        loop {
            let mut tmp: u8 = (val & 0b01111111) as u8;
            val >>= 7;
            if val != 0 {
                tmp |= 0b10000000;
            }
            let n = buf.write(&[tmp]).map_err(|e| Error::Io(e))?;
            n_written += n;
            if n == 0 {
                return Err(Error::BufferTooSmall);
            }
            if val == 0 {
                return Ok(n_written);
            }
        }
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
