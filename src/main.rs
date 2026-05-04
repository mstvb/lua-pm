mod config;
mod downloader;
mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "lua-pm")]
#[command(about = "A simple Lua package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Lua project
    Init,
    /// Add a package
    Add {
        /// URL of the package (git or direct link to zip/tar.gz)
        url: String,
        /// Name of the package
        name: String,
    },
    /// Install all dependencies from lua_pkg.toml
    Install,
    /// Remove a package
    Remove {
        /// Name of the package
        name: String,
    },
    /// List all dependencies
    List,
    /// Run a Lua script
    Run {
        /// Path to the script
        script: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run()?,
        Commands::Add { url, name } => commands::add::run(&url, &name)?,
        Commands::Install => commands::install::run()?,
        Commands::Remove { name } => commands::remove::run(&name)?,
        Commands::List => commands::list::run()?,
        Commands::Run { script } => commands::run::run(&script)?,
    }

    Ok(())
}