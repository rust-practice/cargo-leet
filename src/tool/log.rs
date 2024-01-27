#![cfg(feature = "tool")]
use env_logger::Builder;
use log::LevelFilter;

/// Initializes the logging based on the log level passed
pub fn init_logging(level: LevelFilter) -> anyhow::Result<()> {
    Builder::new().filter(None, level).try_init()?;
    Ok(())
}
