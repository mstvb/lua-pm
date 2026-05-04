use anyhow::Result;
use crate::config::Config;
use crate::downloader::download_package;
use std::path::Path;

pub fn run() -> Result<()> {
    if !Path::new("lua_pkg.toml").exists() {
        println!("No lua_pkg.toml found. Run 'init' first.");
        return Ok(());
    }

    let config = Config::load()?;

    for (name, url) in config.dependencies {
        download_package(&url, &name)?;
    }

    println!("All dependencies installed.");
    Ok(())
}
