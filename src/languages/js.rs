use std::fs;

pub fn update_version(version: String) -> Result<(), Err> {
    let file_path = "package.json";
    let data = fs::read_to_string(file_path)?;
    let updated_data = regex::Regex::new(r#""version": "\d+\.\d+\.\d+""#)?
        .replace(&data, format!(r#""version": "{}""#, version))
        .to_string();
    fs::write(file_path, updated_data)?;
    Ok(())
}