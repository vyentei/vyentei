mod unicode;

use std::{io::{Error, ErrorKind, Result}, env};

fn tasks() -> Result<()> {
    eprintln!("Valid arguments:");
    eprintln!(" — unicode      Fetch updated unicode data from unicode.org");

    Ok(())
}

fn main() -> Result<()> {
    let Some(task) = env::args().nth(1) else {
        tasks()?;

        return Err(Error::new(ErrorKind::Other, "Please specify an argument"));
    };

    match task.as_str() {
        "unicode" => unicode::main(),
        _ => tasks(),
    }
}
