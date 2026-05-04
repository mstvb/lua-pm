use anyhow::{Context, Result};
use mlua::Lua;
use std::fs;

pub fn run(script_path: &str) -> Result<()> {
    let lua = Lua::new();
    
    // Set up package.path to include lua_modules
    let modules_path = "lua_modules/?.lua;lua_modules/?/init.lua;lua_modules/?/?.lua";
    let script = format!(
        "package.path = package.path .. ';{}'\n",
        modules_path
    );
    
    lua.load(&script).exec().context("Failed to set up package.path")?;

    let script_content = fs::read_to_string(script_path)
        .with_context(|| format!("Failed to read script: {}", script_path))?;

    println!("Running {}...", script_path);
    lua.load(&script_content).exec().context("Failed to execute Lua script")?;

    Ok(())
}
