use std::io::Write;
use ::error::Result;


#[derive(Debug)]
pub struct RetrievalResponseItem {
    key: String,
    flags: u16,
    bytes: u32,
    cas_unique: Option<u64>,
    data_block: Vec<u8>,
}


impl RetrievalResponseItem {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(128);
        try!(write!(&mut buf, "VALUE {} {} {}", self.key, self.flags, self.bytes));

        if self.cas_unique.is_some() {
            try!(write!(&mut buf, "{} ", self.cas_unique.unwrap()));
        }
        buf.extend_from_slice(&self.data_block);
        try!(buf.write(b"\r\n"));
        return Ok(buf);
    }
}


pub struct RetrievalResponse(Vec<RetrievalResponseItem>);


impl RetrievalResponse {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(128);
        for item in self.0.iter() {
            buf.extend_from_slice(&try!(item.to_bytes()));
        }
        try!(buf.write(b"END\r\n"));
        return Ok(buf);
    }
}
