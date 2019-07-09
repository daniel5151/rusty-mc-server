use super::types;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadJson(types::string::Error),
}

#[derive(Debug)]
pub enum Status {
    Response { json: String },
    Pong { payload: i64 },
}

impl Status {
    pub fn write<T: std::io::Write>(self, out: &mut T) -> Result<usize, Error> {
        match self {
            Status::Response { json } => {
                types::String::from(json).write(out).map_err(Error::BadJson)
            }
            Status::Pong { payload } => out.write(&i64::to_le_bytes(payload)).map_err(Error::Io),
        }
    }

    pub fn id(&self) -> i32 {
        match self {
            Status::Response { .. } => 0,
            Status::Pong { .. } => 1,
        }
    }
}
