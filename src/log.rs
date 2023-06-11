use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

pub fn init_logging(log_level: LevelFilter) -> anyhow::Result<()> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({M} {l} - {m})}{n}")))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log_level))?;
    log4rs::init_config(config).context("Failed to initialize logging")?;
    Ok(())
}
