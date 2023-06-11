use clap::{Args, Parser, Subcommand, ValueEnum};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Specify the path to the project root (If not provided uses current working directory)
    #[arg(long, short, value_name = "FOLDER")]
    path: Option<String>,

    /// Set logging level to use
    #[arg(long, short, value_enum, default_value_t = LogLevel::Error)]
    pub log_level: LogLevel,
}

impl Cli {
    /// Changes the current working directory to path if one is given
    pub fn set_path(&self) -> anyhow::Result<()> {
        if let Some(path) = &self.path {
            std::env::set_current_dir(path)?
        }
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate(GenerateArgs),
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct GenerateArgs {
    /// Question slug or url
    #[arg(short, long)]
    problem: Option<String>,

    /// Set using question of the day
    #[arg(long, short)]
    daily_challenge: bool,
}

/// Exists to provide better help messages variants copied from LevelFilter as that's the type
/// that is actually needed
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
        Cli::command().debug_assert()
    }
}
