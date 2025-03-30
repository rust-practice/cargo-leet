use std::{
    fmt::Display,
    io::{self, Write},
};

use anyhow::Context;

pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod config_file;
pub(crate) mod core;
pub(crate) mod log;

fn does_user_confirm<S: Display>(prompt: S) -> anyhow::Result<bool> {
    print!("{prompt} (y or yes to confirm) ");
    io::stdout().flush().context("failed to flush stdout")?;

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .context("failed to read user input")?;

    Ok(user_input.to_lowercase().starts_with('y'))
}
