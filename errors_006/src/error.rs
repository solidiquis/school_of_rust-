use std::{
    convert::From,
    error,
    io,
    fmt::{self, Display},
};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    Internal(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(io_err) => writeln!(f, "IO Error: {io_err}"),
            Self::Internal(internal_err) => writeln!(f, "Internal Error: {internal_err}"),
        }
    }
}

impl error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
