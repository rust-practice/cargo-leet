use std::{fs, process::Command};

use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

use crate::tool::config_file::ConfigFile;

pub(crate) fn do_test() -> anyhow::Result<()> {
    let problem_slug = ConfigFile::load()
        .context("failed to load config")?
        .active
        .context("no active problem")?;

    let lib_rs = fs::read_to_string("src/lib.rs").context("failed to read src/lib.rs")?;

    let exp = Regex::new("mod ([a-zA-Z\\-\\_0-9]+);").context("failed to create regex")?;

    let new_lib_rs = lib_rs
        .lines()
        .map(|line| {
            if let Some(m) = exp.captures(line) {
                let name = m.get(1).unwrap().as_str();

                if name == problem_slug.as_str() {
                    return line.to_string();
                }

                format!("// mod {name};")
            } else {
                line.to_string()
            }
        })
        .join("\n");

    fs::write("src/lib.rs", &new_lib_rs).context("failed to write to src/lib.rs")?;

    Command::new("cargo")
        .arg("test")
        .spawn()
        .context("failed to spawn cargo test")?
        .wait()
        .context("failed to wait for cargo test to finish")?;

    fs::write("src/lib.rs", lib_rs).context("failed to restore src/lib.rs")?;

    Ok(())
}
