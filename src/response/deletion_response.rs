use super::response::Response;
use ::error::Result;

#[derive(Debug)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}


impl Response for DeleteResponse {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        use self::DeleteResponse::*;

        Ok(match *self {
            Deleted => b"DELETED\r\n".to_vec(),
            NotFound => b"NOT_FOUND\r\n".to_vec(),
        })
    }
}