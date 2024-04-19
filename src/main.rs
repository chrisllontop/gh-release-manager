mod languages;

use serde::Deserialize;
use std::{env};
use regex::Regex;
use anyhow::{Result, anyhow};

#[derive(Debug, Deserialize)]
struct ActionInput {
    lang: String,
}

fn main() -> Result<()> {
    let lang = env::var("INPUT_LANG")?;
    process_version_update(lang)
}

fn process_version_update(lang: String) -> Result<()> {
    let version = get_release_version()?;
    match lang.as_str() {
        "js" => languages::js::update_version(version),
        "rust" => languages::rust::update_version(version),
        _ => Err(anyhow!("Unsupported language parameter")),
    }
}

fn get_release_version() -> Result<String> {
    let tag = env::var("GITHUB_REF_NAME")?;
    let re = Regex::new(r"^v(\d+\.\d+\.\d+)$").unwrap();
    if let Some(caps) = re.captures(&tag) {
        Ok(caps[1].to_string())
    } else {
        Err(anyhow!("Tag format is invalid"))
    }
}

