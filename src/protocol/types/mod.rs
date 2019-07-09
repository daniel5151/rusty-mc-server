pub mod string;
pub mod varint;

pub use string::String;
pub use varint::VarInt;

#[derive(Debug)]
pub enum Error {
    VarInt(varint::Error),
    String(string::Error),
}
