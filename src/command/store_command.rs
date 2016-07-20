use std::io::BufRead;

use ::error::Result;
use ::error::ErrorKind;
use ::byte_utils::read_until;

#[derive(Debug, PartialEq)]
pub struct StoreCommand {
    pub command_name: String,
    pub key: String,
    pub flags: u16,
    pub exptime: u32,
    pub bytes: u64,
    pub cas_unique: Option<u64>,
    pub noreply: Option<String>,
    pub data_block: Vec<u8>,
}


impl StoreCommand {
    pub fn parse<R: BufRead>(reader: &mut R) -> Result<StoreCommand> {
        let cmd_line = try!(read_until(reader, "\r\n"));
        let cmd_str = try!(String::from_utf8(cmd_line));
        let segments = cmd_str.split_whitespace().collect::<Vec<&str>>();
        let length = segments.len();
        if length < 1 {
            return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into());
        }
        let cmd = segments[0];
        let is_cas = match cmd {
            "set" => false,
            "add" => false,
            "replace" => false,
            "append" => false,
            "prepend" => false,
            "cas" => true,
            _ => return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into()),
        };

        match is_cas {
            true if 6 <= length && length <= 7 => (),
            false if 5 <= length && length <= 6 => (),
            _ => return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into()),
        }

        let bytes = try!(segments[4].parse::<u64>());
        let cas_unique = if is_cas {
            Some(try!(segments[5].parse::<u64>()))
        } else {
            None
        };

        let noreply = match is_cas {
            true if length == 7 => Some(segments[6].to_owned()),
            false if length == 6 => Some(segments[5].to_owned()),
            _ => None,
        };

        let mut data_block = vec![0; bytes as usize];
        let _ = try!(reader.read_exact(&mut data_block));
        if try!(read_until(reader, "\r\n")) != vec![] {
            return Err(ErrorKind::ClientError("error data block".to_owned()).into());
        }

        Ok(StoreCommand {
            command_name: cmd.to_owned(),
            key: segments[1].to_owned(),
            flags: try!(segments[2].parse::<u16>()),
            exptime: try!(segments[3].parse::<u32>()),
            bytes: bytes,
            cas_unique: cas_unique,
            noreply: noreply,
            data_block: data_block,
        })
    }
}
