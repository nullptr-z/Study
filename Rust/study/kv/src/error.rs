use crate::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),
    #[error("Frame`表单: {0} is larger`大于 than max size limit`最大长度: {1}")]
    FrameError(usize, usize),
    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),
    #[error("无法将 {:0} 转换为 {1}")]
    ConvertError(Value, &'static str),
    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {}")]
    StorageError(&'static str, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),
    #[error("Failed to access sled db")]
    SledError(#[from] sled::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Internal`内部的 error: {0}")]
    Internal(String),
}
