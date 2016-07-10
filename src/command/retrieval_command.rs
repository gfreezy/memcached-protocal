use std::io::BufReader;

use ::error::ErrorKind;
use ::error::Result;
use ::byte_utils::read_until;

#[derive(Debug, PartialEq)]
pub struct RetrievalCommand {
    pub command_name: String,
    pub keys: Vec<String>,
}


impl RetrievalCommand {
    pub fn parse(buf: &[u8]) -> Result<RetrievalCommand> {
        let mut reader = BufReader::new(buf);
        let cmd_line = try!(read_until(&mut reader, "\r\n"));
        let cmd_str = try!(String::from_utf8(cmd_line));
        let segments = cmd_str.split_whitespace().collect::<Vec<&str>>();
        let length = segments.len();
        if length < 2 {
            return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into());
        }
        let cmd = segments[0];
        let keys = segments[1..].iter().map(|&s| s.to_owned()).collect();

        Ok(RetrievalCommand {
            command_name: cmd.to_owned(),
            keys: keys,
        })
    }
}
