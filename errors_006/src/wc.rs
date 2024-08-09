#![allow(dead_code)]

use crate::error::{AppError, Result};
use std::{
    convert::AsRef,
    fs,
    path::Path,
};

pub fn file_char_count_v1<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => return Err(AppError::from(err)),
    };

    Ok(content.chars().count())
}

pub fn file_char_count_v2<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)
        .map_err(|e| AppError::IoError(e))?;

    Ok(content.chars().count())
}

pub fn file_char_count_v2_1<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;
    Ok(content.chars().count())
}

pub fn file_char_count_v2_low_carb<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let intermediate = fs::read_to_string(path)
        .map_err(|e| AppError::IoError(e));

    let content = match intermediate {
        Ok(data) => data,
        Err(err) => return Err(err)
    };

    Ok(content.chars().count())
}

pub fn file_char_count_v3<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .map(|data| data.chars().count())
        .map_err(|e| AppError::IoError(e))
}
