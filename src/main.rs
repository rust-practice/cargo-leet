use cargo_leet::{init_logging, run, TopLevel};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let TopLevel::Leet(cli) = TopLevel::parse();
    init_logging(cli.log_level.into())?;
    run(&cli)
}
