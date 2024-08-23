use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub(crate) struct ConfigFile {
    pub active: Option<String>,
}

impl ConfigFile {
    pub(crate) fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(".leet.toml").context("failed to read .leet.toml")?;

        toml::from_str(&content).context("failed to parse .leet.toml")
    }

    pub(crate) fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string(&self).context("failed to convert toml")?;
        fs::write(".leet.toml", content).context("failed to write .leet.toml")?;
        Ok(())
    }
}
