use anyhow::Result;
use crate::config::Config;

pub fn run() -> Result<()> {
    let config = Config::load()?;

    if config.dependencies.is_empty() {
        println!("No dependencies found.");
    } else {
        println!("Dependencies:");
        for (name, url) in config.dependencies {
            println!("  - {}: {}", name, url);
        }
    }

    Ok(())
}
