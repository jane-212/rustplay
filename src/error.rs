use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum IError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    RodioStream(#[from] rodio::StreamError),
    #[error(transparent)]
    RodioPlay(#[from] rodio::PlayError)
}

pub type IResult<T> = Result<T, IError>;

impl From<&str> for IError {
    fn from(cause: &str) -> Self {
        Self::Custom(cause.to_owned())
    }
}

