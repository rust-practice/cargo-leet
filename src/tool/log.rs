#![cfg(feature = "tool")]
use env_logger::Builder;
use log::LevelFilter;

pub fn init_logging(level: LevelFilter) -> anyhow::Result<()> {
    Builder::new().filter(None, level).try_init()?;
    Ok(())
}
