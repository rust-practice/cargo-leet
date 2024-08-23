use std::env;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{debug, info, LevelFilter};

// Based on example <https://docs.rs/clap/latest/clap/_derive/_cookbook/cargo_example_derive/>
// Top level entry point for command line arguments parsing
/// cargo-leet
///
/// cargo-leet is not meant to be used directly, please use `cargo leet` instead
/// without the ` `
#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum TopLevel {
    /// This is necessary because it's a cargo subcommand so the first argument
    /// needs to be the command name
    Leet(Cli),
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Specify the path to the project root (If not provided uses current
    /// working directory)
    #[arg(long, short, global = true, value_name = "FOLDER")]
    path: Option<String>,

    /// Set logging level to use
    #[arg(long, short, global = true, value_enum, default_value_t = LogLevel::Warn)]
    pub log_level: LogLevel,
}

impl Cli {
    /// Changes the current working directory to path if one is given
    pub fn update_current_working_dir(&self) -> anyhow::Result<()> {
        debug!(
            "Before attempting update current dir, it is: {}",
            env::current_dir()?.display()
        );
        if let Some(path) = &self.path {
            info!("Going to update working directory to to '{path}'");
            env::set_current_dir(path)
                .with_context(|| format!("Failed to set current dir to: '{path}'"))?;
            info!(
                "After updating current dir, it is: '{}'",
                env::current_dir()?.display()
            );
        } else {
            debug!("No user supplied path found. No change");
        }
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(visible_alias = "gen", short_flag = 'g')]
    /// Generates the module for the problem
    Generate(GenerateArgs),
    /// Either prints the active problem or sets it to the argument
    Active(ActiveArgs),
    /// Run tests on active problem
    Test,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Question slug or url (If none specified then daily challenge is used)
    pub problem: Option<String>,
    /// If set the module name generated includes the number for the problem
    #[arg(short = 'n', long = "number_in_name", default_value_t = false)]
    pub should_include_problem_number: bool,
}

#[derive(Args, Debug)]
pub struct ActiveArgs {
    pub problem_slug: Option<String>,
}

/// Exists to provide better help messages variants copied from [`LevelFilter`]
/// as that's the type that is actually needed
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LogLevel {
    /// Nothing emitted in this mode
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        // Source: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#testing
        // My understanding it reports most development errors without additional effort
        use clap::CommandFactory;
        TopLevel::command().debug_assert();
    }
}
