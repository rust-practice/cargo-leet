mod generate;
mod helpers;

use self::generate::do_generate;
use crate::tool::cli::{self, Cli};
use anyhow::{bail, Context};
use std::{env, path::Path};

/// Entry point used by the tool. The `main.rs` is pretty thin shim around this
/// function.
///
/// # Errors
/// - Current directory does not exist.
/// - There are insufficient permissions to access the current directory.
/// - Returns an Err if the operation fails.
pub fn run(cli: &Cli) -> anyhow::Result<()> {
    cli.update_current_working_dir()?;

    working_directory_validation()?;

    match &cli.command {
        cli::Commands::Generate(args) => do_generate(args),
    }
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
