use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub(crate) struct ConfigFile {
    pub active: Option<String>,
}

impl ConfigFile {
    const FILENAME: &str = ".leet.toml";

    pub(crate) fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(Self::FILENAME)
            .with_context(|| format!("failed to read {}", Self::FILENAME))?;

        toml::from_str(&content).with_context(|| format!("failed to parse {}", Self::FILENAME))
    }

    pub(crate) fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string(&self).context("failed to convert toml")?;
        fs::write(Self::FILENAME, content)
            .with_context(|| format!("failed to write {}", Self::FILENAME))?;
        Ok(())
    }
}
