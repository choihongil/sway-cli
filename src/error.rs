use std::env;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    JsonError(serde_json::Error),
    VarError(env::VarError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(error)
    }
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error::VarError(error)
    }
}
