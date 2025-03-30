use std::path::Path;

use anyhow::{Context, bail};

use crate::tool::{cli, config_file::ConfigFile};

pub(crate) fn do_active(args: &cli::ActiveArgs) -> anyhow::Result<()> {
    let mut config = ConfigFile::load().unwrap_or_default();
    match &args.problem_slug {
        Some(slug) => {
            if !Path::new(format!("src/{slug}.rs").as_str()).exists() {
                bail!("problem {slug} does not exist");
            }
            if slug == "lib.rs" {
                bail!("cannot set active problem to lib.rs");
            }

            config.active = Some(slug.to_string());
            config.save().context("failed to save config")?;
            println!("Set active problem to {slug}");
        }
        None => match &config.active {
            Some(active) => {
                println!("Active problem: {active}");
            }
            None => {
                println!("No active problem set");
            }
        },
    }

    Ok(())
}
