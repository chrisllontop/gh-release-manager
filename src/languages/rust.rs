use std::fs;
use anyhow::Error;
use regex::Regex;

pub fn update_version(version: String) {
    update_cargo(&version)?;
    update_cargo_lock(&version)?;
}

fn get_project_name() -> Result<String, Error> {
    let data = fs::read_to_string("Cargo.toml")?;
    let re = Regex::new(r#"name = "([^"]+)""#)?;
    if let Some(caps) = re.captures(&data) {
        if let Some(name) = caps.get(1) {
            return Ok(name.as_str().to_string());
        }
    }
    Err(Error::msg("Project name not found in Cargo.toml"))
}

fn update_cargo(version: &str) -> Result<(), Error>{
    let file_path = "Cargo.toml";
    let data = fs::read_to_string(file_path)?;
    let updated_data = regex::Regex::new(r#"version = "\d+\.\d+\.\d+""#)?
        .replace(&data, format!(r#"version = "{}""#, version))
        .to_string();
    fs::write(file_path, updated_data)?;
    Ok(())
}

fn update_cargo_lock(version: &str) -> Result<(), Error> {
    let project_name = get_project_name()?;
    let file_path = "Cargo.lock";
    let data = fs::read_to_string(file_path)?;
    let regex_pattern = format!(r#"\[\[package\]\]\nname = "{}"\nversion = "\d+\.\d+\.\d+""#, project_name);
    let regex = Regex::new(&regex_pattern)?;
    let updated_data = regex.replace(&data, format!(r#"[[package]]\nname = "{}"\nversion = "{}""#, project_name, version)).to_string();
    fs::write(file_path, updated_data)?;
    Ok(())
}