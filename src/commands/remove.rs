use anyhow::Result;
use crate::config::Config;
use std::fs;
use std::path::Path;

pub fn run(name: &str) -> Result<()> {
    let mut config = Config::load()?;

    if config.dependencies.remove(name).is_some() {
        let package_dir = Path::new("lua_modules").join(name);
        if package_dir.exists() {
            fs::remove_dir_all(package_dir)?;
        }
        config.save()?;
        println!("Removed dependency: {}", name);
    } else {
        println!("Dependency {} not found", name);
    }

    Ok(())
}
