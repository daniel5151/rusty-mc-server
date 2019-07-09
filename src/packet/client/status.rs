#[derive(Debug)]
pub enum Error {
    InvalidId(i32),
    Io(std::io::Error),
}

#[derive(Debug)]
pub enum Status {
    Request,
    Ping { payload: i64 },
}

impl Status {
    fn read_request<T: std::io::Read>(_inbuf: &mut T) -> Result<Status, Error> {
        // nothing to read
        Ok(Status::Request)
    }

    fn read_ping<T: std::io::Read>(inbuf: &mut T) -> Result<Status, Error> {
        let mut long_buf = [0; 8];
        inbuf.read(&mut long_buf).map_err(Error::Io)?;
        let payload = i64::from_le_bytes(long_buf);

        Ok(Status::Ping { payload })
    }

    pub fn read<T: std::io::Read>(id: i32, inbuf: &mut T) -> Result<Status, Error> {
        match id {
            0 => Self::read_request(inbuf),
            1 => Self::read_ping(inbuf),
            _ => return Err(Error::InvalidId(id)),
        }
    }
}
