use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Specify the path to the project root (If not provided uses current working directory)
    #[arg(long, short, value_name = "FOLDER")]
    path: Option<String>,
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
