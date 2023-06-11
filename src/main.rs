use cargo_leet::{init_logging, run, CargoCli};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let CargoCli::Leet(cli) = CargoCli::parse();
    init_logging(cli.log_level.into())?;
    run(&cli)
}
