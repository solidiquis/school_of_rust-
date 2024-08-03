#![allow(dead_code)]

use std::{
    io::Error,
    fs::read_to_string,
};

fn count_lines(path: &str) -> usize {
    match read_to_string(path) {
        Ok(txt) => txt.lines().count(),
        Err(err) => panic!("{err}"),
    }
}

fn count_lines_v2(path: &str) -> Result<usize, Error> {
    match read_to_string(path) {
        Ok(txt) => Ok(txt.lines().count()),
        Err(err) => return Err(err)
    }
}

fn count_lines_v3(path: &str) -> Result<usize, Error> {
    read_to_string(path).map(|txt| txt.lines().count())
}

// Totally equivalent to v2
fn count_lines_v4(path: &str) -> Result<usize, Error> {
    let data = read_to_string(path)?;
    Ok(data.lines().count())
}

// Totally equivalent to version 1
fn count_lines_v5(path: &str) -> usize {
    read_to_string(path).unwrap().lines().count()
}

fn count_lines_v6(path: &str) -> usize {
    if let Ok(txt) = read_to_string(path) {
        txt.lines().count()
    } else {
        panic!("uh oh")
    }
}

fn count_lines_v7(path: &str) -> usize {
    let Ok(txt) = read_to_string(path) else {
        panic!("oh no");
    };
    txt.lines().count()
}

// Count the amount of lines in a file.
fn main() {
    count_lines("foo.txt");

    let foo = 1;

    match foo {
        1 => println!("1"),
        2 => println!("2"),
        3..=100 => println!("3..=100"),
        _ => println!("{foo}"),
    }
}
