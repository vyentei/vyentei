use std::{
    fmt::Write as _,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    process::Command,
};

use crate::{Error, ErrorKind, Result};

const GENERATED_PATH: &str = "./generated/";

pub fn main() -> Result {
    println!("Generating QuantiiSans-Oblique…");
    fs::remove_dir_all(GENERATED_PATH).ok();
    fs::create_dir_all(GENERATED_PATH)?;
    Command::new("python3")
        .arg("./tools/python/generate-oblique.py")
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                "Failed to generate QuantiiSans-Oblique!",
            )
        })?;

    let mut output = BufWriter::new(File::create(
        "./generated/QuantiiSans-Oblique.sfd.tmp",
    )?);

    for line in
        BufReader::new(File::open("./generated/QuantiiSans-Oblique.sfd")?)
            .lines()
    {
        let line = line?;

        if line.starts_with("Refer: ") {
            let items: Vec<&str> = line.split(' ').collect();
            let x: i32 = items[items.len() - 3].parse().map_err(|e| {
                println!("Invalid line: {line}");

                Error::new(ErrorKind::Other, e)
            })?;
            let x = x + 89;
            let mut line = String::new();

            for item in items.iter().take(items.len() - 3) {
                write!(&mut line, "{item} ").unwrap();
            }

            write!(
                &mut line,
                "{x} {} {}",
                items[items.len() - 2],
                items[items.len() - 1],
            )
            .unwrap();

            writeln!(&mut output, "{line}")?;
        } else if line.starts_with("FontName: ") {
            writeln!(&mut output, "{line}-Oblique")?;
        } else if line.starts_with("FullName: ") {
            writeln!(&mut output, "{line} Oblique")?;
        } else if line.starts_with("LangName: ") {
            let items: Vec<&str> = line.split(' ').rev().skip(1).collect();

            for item in items.iter().rev() {
                write!(&mut output, "{item} ")?;
            }

            writeln!(&mut output, "\"Oblique\"")?;
        } else {
            writeln!(&mut output, "{line}")?;
        }
    }

    fs::rename(
        "./generated/QuantiiSans-Oblique.sfd.tmp",
        "./generated/QuantiiSans-Oblique.sfd",
    )?;

    println!("Generated QuantiiSans-Oblique!");
    println!("Generating OTF files…");
    Command::new("python3")
        .arg("./tools/python/generate-otf.py")
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| {
            Error::new(ErrorKind::Other, "Failed to generate OTF files!")
        })?;
    println!("Generated OTF files!");

    Ok(())
}
