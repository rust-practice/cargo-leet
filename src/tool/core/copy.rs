use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result};
use arboard::Clipboard;
use itertools::Itertools;
use log::{debug, info};

use super::generate::SEPARATOR;

pub(crate) fn copy() -> Result<()> {
    // Logging
    info!("Starting copy function");

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

    debug!("Found file: {:?}", file_name);

    let file_path = src_dir.join(file_name);
    let file = fs::File::open(&file_path)
        .context(format!("Couldn't open file {}", file_path.display()))?;
    let reader = BufReader::new(file);

    // Logging
    debug!("Reading file contents");

    let contents: String = reader
        .lines()
        .take_while(|line| {
            line.as_ref()
                .map_or(false, |line| !line.contains(SEPARATOR))
        })
        .filter_map(Result::ok)
        .join("\n");

    debug!("File contents read ({} bytes)", contents.len());

    let mut clipboard = Clipboard::new().context("Couldn't initialize clipboard")?;
    clipboard
        .set_text(contents)
        .context("Couldn't set text to clipboard")?;

    // Logging
    info!("Copied contents to clipboard");

    Ok(())
}
