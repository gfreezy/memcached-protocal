use std::io::BufReader;

use ::error::ErrorKind;
use ::error::Result;
use ::byte_utils::read_until;

#[derive(Debug, PartialEq)]
pub struct DeletionCommand {
    pub command_name: String,
    pub key: String,
    pub noreply: Option<String>,
}


impl DeletionCommand {
    pub fn parse(buf: &[u8]) -> Result<DeletionCommand> {
        let mut reader = BufReader::new(buf);
        let cmd_line = try!(read_until(&mut reader, "\r\n"));
        let cmd_str = try!(String::from_utf8(cmd_line));
        let segments = cmd_str.split_whitespace().collect::<Vec<&str>>();
        let length = segments.len();
        if length < 2 {
            return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into());
        }
        let cmd = segments[0];
        let key = segments[1];
        let noreply = if length > 2 {
            Some(segments[2].to_string())
        } else {
            None
        };

        Ok(DeletionCommand{
            command_name: cmd.to_owned(),
            key: key.to_owned(),
            noreply: noreply,
        })
    }
}
