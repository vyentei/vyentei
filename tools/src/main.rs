mod generate;
mod unicode;

use std::{
    env,
    io::{Error, ErrorKind},
};

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn tasks() -> Result {
    eprintln!("Valid arguments:");
    eprintln!(" — unicode      Fetch updated unicode data from unicode.org");
    eprintln!(" — generate     Generate font variants and OTF output");

    Ok(())
}

fn main() -> Result {
    let Some(task) = env::args().nth(1) else {
        tasks()?;

        return Err(Error::new(ErrorKind::Other, "Please specify an argument"));
    };

    match task.as_str() {
        "unicode" => unicode::main(),
        "generate" => generate::main(),
        _ => tasks(),
    }
}
