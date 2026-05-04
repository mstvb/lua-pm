use anyhow::Result;
use crate::config::Config;
use crate::downloader::download_package;

pub fn run(url: &str, name: &str) -> Result<()> {
    let mut config = Config::load()?;

    config.dependencies.insert(name.to_string(), url.to_string());
    
    download_package(url, name)?;

    config.save()?;
    println!("Added dependency: {} from {}", name, url);
    Ok(())
}
