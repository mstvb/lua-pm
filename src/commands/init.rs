use anyhow::Result;
use std::path::Path;
use crate::config::{Config, ProjectInfo};

pub fn run() -> Result<()> {
    let config_path = Path::new("lua_pkg.toml");
    if config_path.exists() {
        println!("lua_pkg.toml already exists");
        return Ok(());
    }

    let config = Config {
        project: ProjectInfo {
            name: "my_lua_project".to_string(),
            version: "0.1.0".to_string(),
        },
        ..Default::default()
    };

    config.save()?;
    println!("Initialized lua_pkg.toml");
    Ok(())
}
