use ::error::Result;

use super::response::Response;


#[derive(Debug)]
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}


impl Response for StoreResponse {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        use self::StoreResponse::*;

        Ok(match *self {
            Stored => b"STORED\r\n".to_vec(),
            NotStored => b"NOT_STORED\r\n".to_vec(),
            Exists => b"EXISTS\r\n".to_vec(),
            NotFound => b"NOT_FOUND\r\n".to_vec(),
        })
    }
}