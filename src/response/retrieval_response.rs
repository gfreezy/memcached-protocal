use std::io::Write;
use ::error::Result;

use super::response::Response;

#[derive(Debug)]
pub struct RetrievalResponseItem {
    pub key: String,
    pub flags: u16,
    pub bytes: u32,
    pub cas_unique: Option<u64>,
    pub data_block: Vec<u8>,
}


impl Response for RetrievalResponseItem {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(128);
        try!(write!(&mut buf, "VALUE {} {} {} ", self.key, self.flags, self.bytes));

        if self.cas_unique.is_some() {
            try!(write!(&mut buf, "{} ", self.cas_unique.unwrap()));
        }
        buf.extend_from_slice(&self.data_block);
        try!(buf.write(b"\r\n"));
        return Ok(buf);
    }
}


pub struct RetrievalResponse(pub Vec<RetrievalResponseItem>);


impl Response for RetrievalResponse {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(128);
        for item in self.0.iter() {
            buf.extend_from_slice(&try!(item.to_bytes()));
        }
        try!(buf.write(b"END\r\n"));
        return Ok(buf);
    }
}
