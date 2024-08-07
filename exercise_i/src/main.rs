use std::{
    env,
    fs,
    process::ExitCode
};

/// Application-specific error types
mod error;
use error::Error;

fn main() -> ExitCode {
    match run() {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<String, Error> {
    let clargs = env::args();

    if clargs.len() < 2 {
        return Err(Error::NotEnoughArgument(clargs.len()))
    }

    let mut output = String::new();


    for arg in clargs.skip(1) {
        let data = fs::read_to_string(&arg).map_err(Error::IoError)?;
        let num_lines = data.lines().count();
        output += &format!("{arg} {num_lines}\n");
    }

    Ok(output)
}
