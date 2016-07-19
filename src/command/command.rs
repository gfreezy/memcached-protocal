use std::io::BufRead;
use super::deletion_command::DeletionCommand;
use super::retrieval_command::RetrievalCommand;
use super::store_command::StoreCommand;

use ::error::Result;
use ::error::ErrorKind;


#[derive(Debug, PartialEq)]
pub enum Command {
    Delete(DeletionCommand),
    Retrieval(RetrievalCommand),
    Store(StoreCommand),
}


pub fn parse<R: BufRead>(reader: &mut R) -> Result<Command> {
    use self::Command::*;

    let cmd_str;
    {
        let buf = try!(reader.fill_buf());
        cmd_str = String::from_utf8_lossy(buf).into_owned();
    }
    let segments = cmd_str.split_whitespace().collect::<Vec<&str>>();
    let length = segments.len();
    if length < 1 {
        return Err(ErrorKind::ClientError("wrong size of params".to_owned()).into());
    }
    let cmd = segments[0];
    Ok(match cmd {
        "set" | "add" | "replace" | "append" | "prepend" | "cas" => Store(try!(StoreCommand::parse(reader))),
        "get" | "gets" => Retrieval(try!(RetrievalCommand::parse(reader))),
        "delete" => Delete(try!(DeletionCommand::parse(reader))),
        _ => return Err(ErrorKind::ClientError("not supported command".to_owned()).into()),
    })
}
