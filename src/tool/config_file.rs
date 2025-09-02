use std::fs;

use anyhow::Context;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct ConfigFile {
    pub active: Option<String>,
    pub should_include_problem_number_in_mod_name: bool,
}

impl ConfigFile {
    const FILENAME: &str = ".leet.toml";

    pub(crate) fn load() -> anyhow::Result<Self> {
        let content = match std::fs::read_to_string(Self::FILENAME) {
            Ok(x) => x,
            Err(e) => {
                info!(
                    "failed to load {:?}. Using defaults. Error msg: {e:?}",
                    Self::FILENAME
                );
                return Ok(Self::default());
            }
        };

        toml::from_str(&content).with_context(|| format!("failed to parse {}", Self::FILENAME))
    }

    pub(crate) fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string(&self).context("failed to convert toml")?;
        fs::write(Self::FILENAME, content)
            .with_context(|| format!("failed to write {}", Self::FILENAME))?;
        Ok(())
    }
}
