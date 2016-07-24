use ::error::Result;

pub trait Response {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}