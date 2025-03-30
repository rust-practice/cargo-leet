mod active;
mod generate;
mod helpers;
mod new;
mod test;

use self::generate::do_generate;
use crate::tool::cli::{self, Cli};
use active::do_active;
use anyhow::{bail, Context};
use new::do_new;
use std::{env, path::Path};
use test::do_test;

/// Entry point used by the tool. The `main.rs` is pretty thin shim around this
/// function.
///
/// # Errors
/// Too numerous to mention. ;-)
pub fn run(cli: &Cli) -> anyhow::Result<()> {
    cli.update_current_working_dir()?;

    match &cli.command {
        cli::Commands::Generate(args) => {
            working_directory_validation()?;
            do_generate(args)
        }
        cli::Commands::Active(args) => {
            working_directory_validation()?;
            do_active(args)
        }
        cli::Commands::Test => {
            working_directory_validation()?;
            do_test()
        }
        cli::Commands::New(args) => do_new(args),
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
