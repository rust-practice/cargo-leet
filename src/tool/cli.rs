use std::env;

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{debug, info, LevelFilter};

/// Top level entry point for command line arguments parsing
///
/// Based on example <https://docs.rs/clap/latest/clap/_derive/_cookbook/cargo_example_derive/>
#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    // This is necessary because it is a cargo subcommand so the first argument needs to be the
    // command name
    /// A program that given the link or slug to a leetcode problem, creates a
    /// local file where you can develop and test your solution before posting it
    /// back to leetcode.
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
    pub path: Option<String>,

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
            std::env::set_current_dir(path)
                .with_context(|| format!("Failed to set current dir to: '{path}'"))?;
            info!(
                "After updating current dir, it is: '{}'",
                env::current_dir()?.display()
            );
        } else {
            debug!("No user supplied path found. No change")
        }
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(visible_alias = "gen", short_flag = 'g')]
    Generate(GenerateArgs),

    /// Creates or updates a project
    #[clap(short_flag = 'i')]
    Init(InitArgs),
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
pub struct InitArgs {
    /// Do not make any changes only show what changes would be made
    #[arg(short = 'd', long, default_value_t = false)]
    pub dry_run: bool,
}

/// Exists to provide better help messages variants copied from LevelFilter as
/// that's the type that is actually needed
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
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
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
        CargoCli::command().debug_assert()
    }
}
