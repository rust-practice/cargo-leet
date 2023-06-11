use cargo_leet::{cli::Cli, run};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(&cli)
}
