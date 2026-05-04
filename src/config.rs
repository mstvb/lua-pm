use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub project: ProjectInfo,
    pub dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        if Path::new("lua_pkg.toml").exists() {
            let content = fs::read_to_string("lua_pkg.toml")?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::write("lua_pkg.toml", toml)?;
        Ok(())
    }
}
