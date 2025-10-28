use dirs::data_dir;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{fs, path::PathBuf};

fn get_project_name() -> String {
    let cargo_toml = include_str!("../../Cargo.toml");
    for line in cargo_toml.lines() {
        if let Some(name) = line.strip_prefix("name = \"") {
            if let Some(name) = name.strip_suffix("\"") {
                return name.to_string();
            }
        }
    }
    panic!("Could not find project name in Cargo.toml");
}

fn get_path(key: &str) -> PathBuf {
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(get_project_name());
    fs::create_dir_all(&path).ok();
    path.push(format!("{key}.json"));
    path
}

pub fn save<T: Serialize>(key: &str, value: &T) {
    let json = serde_json::to_string_pretty(value).expect("Could not serialize");
    fs::write(get_path(key), json).expect("Could not write to file");
}

pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let data = fs::read_to_string(get_path(key)).ok()?;
    serde_json::from_str(&data).ok()
}
