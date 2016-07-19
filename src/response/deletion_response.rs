
#[derive(Debug)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}


impl DeleteResponse {
    pub fn to_bytes(&self) -> &'static [u8] {
        use self::DeleteResponse::*;

        match *self {
            Deleted => b"DELETED\r\n",
            NotFound => b"NOT_FOUND\r\n",
        }
    }
}