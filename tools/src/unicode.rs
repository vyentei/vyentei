use std::{io::Result, fs, process::Command};

const UNIDATA_PATH: &str = "./external/UNIDATA";

pub fn main() -> Result<()> {
    println!("Downloading unicode data…");

    fs::remove_dir_all(UNIDATA_PATH).ok();

    fs::create_dir_all(UNIDATA_PATH)?;

    // FIXME: Use fetchy (curl crate backend), with ETag to avoid re-downloads
    //
    // Then, can merge with unicover
    Command::new("wget")
        .arg("-P")
        .arg("./external/UNIDATA")
        .arg("http://www.unicode.org/Public/UNIDATA/UnicodeData.txt")
        .arg("http://www.unicode.org/Public/UNIDATA/Blocks.txt")
        .output()?;
      
    println!("Downloaded unicode data!");

    Ok(())
}
