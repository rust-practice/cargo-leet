mod generate;
mod helpers;

use self::generate::do_generate;
use crate::tool::cli::{self, Cli};
use anyhow::{bail, Context};
use std::{env, path::Path};

use super::cli::InitArgs;

/// Entry point used by the tool. The `main.rs` is pretty thin shim around this
/// function.
pub fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        cli::Commands::Generate(args) => {
            cli.update_current_working_dir()?;
            working_directory_validation()?;
            do_generate(args)
        }
        cli::Commands::Init(args) => do_init(cli.path.as_ref(), args),
    }
}

fn do_init(path: Option<&String>, args: &InitArgs) -> Result<(), anyhow::Error> {
    dbg!(path, args);
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
