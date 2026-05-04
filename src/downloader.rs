use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use reqwest::blocking::Client;

fn is_github_repo(url: &str) -> bool {
    // Matches "owner/repo" or "https://github.com/owner/repo"
    if url.starts_with("http://") || url.starts_with("https://") {
        url.contains("github.com") && url.split('/').filter(|s| !s.is_empty()).count() >= 3
    } else {
        let parts: Vec<&str> = url.split('/').collect();
        // Just owner/repo, potentially with .lua in repo name
        parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() && !parts[0].contains('.')
    }
}

fn get_github_zip_url(url: &str) -> String {
    if url.starts_with("http") {
        let trimmed_url = url.trim_end_matches(".git");
        let parts: Vec<&str> = trimmed_url.split('/').filter(|s| !s.is_empty()).collect();
        // parts should be ["https:", "github.com", "owner", "repo"]
        if parts.len() >= 4 {
            return format!("https://github.com/{}/{}/archive/refs/heads/master.zip", parts[2], parts[3]);
        }
    } else {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() == 2 {
            return format!("https://github.com/{}/{}/archive/refs/heads/master.zip", parts[0], parts[1]);
        }
    }
    url.to_string()
}

pub fn download_package(url: &str, name: &str) -> Result<()> {
    let modules_dir = Path::new("lua_modules");
    if !modules_dir.exists() {
        fs::create_dir(modules_dir)?;
    }

    let package_dir = modules_dir.join(name);
    if package_dir.exists() {
        fs::remove_dir_all(&package_dir)?;
    }
    fs::create_dir(&package_dir)?;

    let mut final_url = url.to_string();
    let mut is_github = false;

    if is_github_repo(url) {
        if url.starts_with("http") {
             if !url.ends_with(".lua") && !url.ends_with(".zip") && !url.ends_with(".tar.gz") {
                 final_url = get_github_zip_url(url);
                 is_github = true;
             }
        } else {
             final_url = get_github_zip_url(url);
             is_github = true;
        }
    }

    println!("Downloading {}...", final_url);
    
    let client = Client::builder()
        .user_agent("lua-pm (Rust)")
        .build()?;
    
    let mut response = client.get(&final_url).send().context("Failed to download package")?;
    
    if !response.status().is_success() && is_github && final_url.contains("master.zip") {
        // Try main branch if master fails
        let main_url = final_url.replace("master.zip", "main.zip");
        println!("Master branch failed, trying main branch: {}...", main_url);
        response = client.get(&main_url).send().context("Failed to download package from main branch")?;
        if response.status().is_success() {
            final_url = main_url;
        }
    }

    if !response.status().is_success() {
        bail!("Failed to download package: HTTP {}", response.status());
    }

    if final_url.ends_with(".zip") {
        let mut tmp_file = tempfile::tempfile()?;
        response.copy_to(&mut tmp_file)?;
        let mut archive = zip::ZipArchive::new(tmp_file)?;
        
        if is_github {
            // GitHub zips have a root folder like repo-master/
            // We want to extract its contents directly into package_dir
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = match file.enclosed_name() {
                    Some(path) => {
                        // Skip the first component
                        let mut components = path.components();
                        components.next(); 
                        components.as_path().to_path_buf()
                    },
                    None => continue,
                };

                if outpath.as_os_str().is_empty() {
                    continue;
                }

                let full_path = package_dir.join(outpath);

                if (*file.name()).ends_with('/') {
                    fs::create_dir_all(&full_path)?;
                } else {
                    if let Some(p) = full_path.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = fs::File::create(&full_path)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
        } else {
            archive.extract(&package_dir)?;
        }
    } else if final_url.ends_with(".tar.gz") {
        let tar_gz = flate2::read::GzDecoder::new(response);
        let mut archive = tar::Archive::new(tar_gz);
        archive.unpack(&package_dir)?;
    } else {
        let file_name = if final_url.ends_with(".lua") {
             final_url.split('/').last().unwrap_or("module.lua")
        } else {
            "init.lua"
        };
        let mut dest = fs::File::create(package_dir.join(file_name))?;
        response.copy_to(&mut dest)?;
    }

    println!("Installed {} to lua_modules/{}", name, name);
    Ok(())
}
