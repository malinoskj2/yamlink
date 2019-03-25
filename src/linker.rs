#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::error::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub type DIResult<T> = Result<T, Box<Error>>;

const CFG_MAP_NAME: &str = "config-map.yaml";

pub fn find_config_map(repo_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name() == OsString::from(CFG_MAP_NAME))
        .map(DirEntry::into_path)
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
enum CLMethod {
    link,
    copy,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigLink {
    name: String,
    source: PathBuf,
    destination: PathBuf,
    method: CLMethod,
}