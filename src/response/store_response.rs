
#[derive(Debug)]
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}


impl StoreResponse {
    pub fn to_bytes(&self) -> &'static [u8] {
        use self::StoreResponse::*;

        match *self {
            Stored => b"STORED\r\n",
            NotStored => b"NOT_STORED\r\n",
            Exists => b"EXISTS\r\n",
            NotFound => b"NOT_FOUND\r\n",
        }
    }
}