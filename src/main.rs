use cargo_leet::{init_logging, run, Cli};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_logging(cli.log_level.into())?;
    run(&cli)
}
