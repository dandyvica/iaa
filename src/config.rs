// manage configuration from the config.toml file

use std::{collections::HashMap, path::Path, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    artfeact: HashMap<String, Artefact>,
}

#[derive(Debug, Deserialize)]
struct Artefact {
    discover: bool,
    category: String,
}

impl TryFrom<&Path> for Config {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        // load file as a string
        let cfg = std::fs::read_to_string(path)?;

        // convert .toml to Config struct
        let config = toml::from_str(&cfg)?;

        Ok(config)
    }
}
