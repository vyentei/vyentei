use std::{
    fmt::Write as _,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    process::Command,
};

use crate::{Error, ErrorKind, Result};

const GENERATED_PATH: &str = "./generated/";

fn oblique(script_name: &str, font_name: &str, marks: &str, shift: i32) -> Result {
    Command::new("python3")
        .arg(script_name)
        .arg(marks)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to generate {font_name}!"),
            )
        })?;

    let mut output = BufWriter::new(File::create(format!(
        "./generated/{font_name}.sfd.tmp"
    ))?);
    let mut should_shift = true;

    for line in
        BufReader::new(File::open(format!("./generated/{font_name}.sfd"))?)
            .lines()
    {
        let line = line?;

        if line.starts_with("Refer: ") && should_shift {
            let items: Vec<&str> = line.split(' ').collect();
            let x: i32 = items[items.len() - 3].parse().map_err(|e| {
                println!("Invalid line: {line}");

                Error::new(ErrorKind::Other, e)
            })?;
            let y: i32 = items[items.len() - 2].parse().map_err(|e| {
                println!("Invalid line: {line}");

                Error::new(ErrorKind::Other, e)
            })?;
            let x = x + shift;
            let mut line = String::new();

            for item in items.iter().take(items.len() - 3) {
                write!(&mut line, "{item} ").unwrap();
            }

            write!(&mut line, "{x} {y} {}", items[items.len() - 1]).unwrap();

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
            if let Some(codepoint) = line.strip_prefix("StartChar: uni") {
                should_shift =
                    if let Ok(bytes) = u32::from_str_radix(codepoint, 16) {
                        char::from_u32(bytes).is_some()
                    } else {
                        false
                    };
            } else if line.starts_with("StartChar: ") {
                should_shift = true;
            }

            writeln!(&mut output, "{line}")?;
        }
    }

    fs::rename(
        format!("./generated/{font_name}.sfd.tmp"),
        format!("./generated/{font_name}.sfd"),
    )
}

pub fn main() -> Result {
    println!("Clearing generated folder…");
    fs::remove_dir_all(GENERATED_PATH).ok();
    fs::create_dir_all(GENERATED_PATH)?;

    println!("Generating list of marks…");

    let mut marks = String::new();

    for line in
        BufReader::new(File::open("./external/UNIDATA/UnicodeData.txt")?)
            .lines()
    {
        let line = line?;
        let mut iter = line.split(';');
        let codepoint = iter.next().ok_or_else(|| {
            println!("Invalid line: {line}");

            Error::new(ErrorKind::Other, "codepoint")
        })?;
        let category = iter.skip(1).next().ok_or_else(|| {
            println!("Invalid line: {line}");

            Error::new(ErrorKind::Other, "category")
        })?;

        if matches!(category, "Sk" | "Mn" | "Me" | "Mc") {
            let codepoint =
                u32::from_str_radix(codepoint, 16).map_err(|e| {
                    println!("Invalid line: {line}");

                    Error::new(ErrorKind::Other, e)
                })?;

            write!(marks, "{codepoint} ").unwrap();
        }
    }

    marks.pop();
    println!("Generating QuantiiSans-Oblique…");
    oblique(
        "./tools/python/generate-oblique.py",
        "QuantiiSans-Oblique",
        marks.as_str(),
        89,
    )?;
    println!("Generated QuantiiSans-Oblique!");
    println!("Generating QuantiiSansMono-Oblique…");
    oblique(
        "./tools/python/generate-mono-oblique.py",
        "QuantiiSansMono-Oblique",
        "",
        89,
    )?;
    println!("Generated QuantiiSansMono-Oblique!");
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

#[cfg(test)]
mod tests {
    const SLANT_MATRIX: [[f64; 2]; 2] =
        [[1.0, 0.0], [0.17632698070846498, 1.0]];

    fn matrix_inv(matrix: [[f64; 2]; 2]) -> [[f64; 2]; 2] {
        let [[a, b], [c, d]] = matrix;
        let div = a * d - b * c;
        let mul = div.recip();

        [[mul * d, -mul * b], [-mul * c, mul * a]]
    }

    fn vector_mul_matrix(vector: [f64; 2], matrix: [[f64; 2]; 2]) -> [f64; 2] {
        [
            matrix[0][0] * vector[0] + matrix[1][0] * vector[1],
            matrix[0][1] * vector[0] + matrix[1][1] * vector[1],
        ]
    }

    /*fn vector_round(vector: [f64; 2]) -> [i32; 2] {
        [vector[0].round() as i32, vector[1].round() as i32]
    }*/

    #[test]
    fn inverse() {
        let matrix = SLANT_MATRIX;
        let inverse = matrix_inv(SLANT_MATRIX);
        let vectors = [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0], [0.0, 1.0]];

        for [x, y] in vectors {
            let orig = [x, y];
            let [x, y] = vector_mul_matrix([x, y], inverse);
            let vector = vector_mul_matrix([x, y], matrix);

            assert_eq!(vector, orig);
        }
    }

    #[test]
    fn slant() {
        let matrix = SLANT_MATRIX;
        let vectors = [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0], [0.0, 1.0]];

        for [x, y] in vectors {
            let [ox, oy] = [x, y];
            let [x, y] = vector_mul_matrix([x, y], matrix);

            println!("({ox}, {oy}) ({x}, {y})");
        }
    }
}
