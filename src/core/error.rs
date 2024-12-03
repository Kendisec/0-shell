use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}