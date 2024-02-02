#![cfg(feature = "tool")]
use env_logger::Builder;
use log::LevelFilter;

/// Initializes the logging based on the log level passed
///
/// # Errors
/// This function will fail if it is called more than once, or if another
/// library has already initialized a global logger.
pub fn init_logging(level: LevelFilter) -> anyhow::Result<()> {
    Builder::new().filter(None, level).try_init()?;
    Ok(())
}
