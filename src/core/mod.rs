use std::{env, path::Path};

use anyhow::{bail, Context};

use crate::cli::Cli;

mod code_snippet;
mod daily_challenge;
mod write_file;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    cli.update_current_working_dir()?;

    working_directory_validation()?;

    // let mut args = std::env::args();
    // let title_slug = if args.len() == 1 {
    //     daily_challenge::get_daily_challenge_slug()
    // } else if args.len() != 2 {
    //     return Err("Usage: binary SLUG".into());
    // } else {
    //     args.nth(1).unwrap()
    // };
    // let code_snippet = code_snippet::generate_code_snippet(&title_slug);
    // write_file::write_file(&title_slug, code_snippet)?;
    Ok(())
}

fn working_directory_validation() -> anyhow::Result<()> {
    let req_file = "Cargo.toml";
    let path = Path::new(req_file);
    if path.exists() {
        Ok(())
    } else {
        bail!(
            "Failed to find {req_file} in current directory '{}'",
            env::current_dir()
                .context("Failed to get current working directory")?
                .display()
        );
    }
}
