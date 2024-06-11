use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result};
use arboard::Clipboard;
use itertools::Itertools;

use super::generate::SEPARATOR;

pub(crate) fn copy() -> Result<()> {
    // Find the first file in src/ that is not lib.rs
    let src_dir = Path::new("src");
    let file_name = fs::read_dir(src_dir)
        .context("Couldn't read src/ directory")?
        .find_map(|entry| {
            let entry = entry
                .context("Error reading entry in src/ directory")
                .ok()?;
            let name = entry.file_name();
            if name == "lib.rs" {
                None
            } else {
                Some(name)
            }
        })
        .context("No file beside lib.rs found in src/")?;

    let file_path = src_dir.join(file_name);
    let file = fs::File::open(&file_path)
        .context(format!("Couldn't open file {}", file_path.display()))?;
    let reader = BufReader::new(file);

    let contents: String = reader
        .lines()
        .take_while(|line| {
            line.as_ref()
                .map_or(false, |line| !line.contains(SEPARATOR))
        })
        .filter_map(Result::ok)
        .join("\n");

    let mut clipboard = Clipboard::new().context("Couldn't initialize clipboard")?;
    clipboard
        .set_text(contents)
        .context("Couldn't set text to clipboard")?;

    Ok(())
}
