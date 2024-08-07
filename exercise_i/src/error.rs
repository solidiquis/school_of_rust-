use std::{
    error::Error as StdError,
    io,
    fmt,
};

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    NotEnoughArgument(usize),
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument => writeln!(f, "Invalid argument"),
            Self::NotEnoughArgument(num) => {
                writeln!(f, "Expected at least 2 arguments but got {num}")
            }
            Self::IoError(io_err) => writeln!(f, "{io_err}")
        }
    }
}

impl StdError for Error {}
