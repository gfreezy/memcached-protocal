#[macro_use]
extern crate error_chain;

mod command;
mod error;
mod byte_utils;
mod response;

pub use command::store_command::StoreCommand;
pub use command::retrieval_command::RetrievalCommand;
pub use command::deletion_command::DeletionCommand;
pub use command::command::parse;
pub use command::command::Command::Delete;
pub use command::command::Command::Retrieval;
pub use command::command::Command::Store;
pub use error::Result;

pub use response::deletion_response::DeleteResponse;
pub use response::retrieval_response::RetrievalResponse;
pub use response::retrieval_response::RetrievalResponseItem;
pub use response::store_response::StoreResponse;
