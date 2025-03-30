use cargo_leet::{TopLevel, init_logging, run};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let TopLevel::Leet(cli) = TopLevel::parse();
    init_logging(cli.log_level.into())?;
    run(&cli)
}
