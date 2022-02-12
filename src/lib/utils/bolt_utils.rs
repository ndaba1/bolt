use std::fs::read_to_string;
use std::path::Path;

use super::fs_utils;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceConfig {
    pub workspace: String,
    pub source: Vec<String>,
    pub registered: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub info: Reference,
    pub policies: Vec<Policy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    pub name: String,
    pub alias: String,
    pub priority: usize,
    pub env: Envrionment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envrionment {
    pub from: String,
    pub cli: Option<String>,
    pub pkg_mgr: PkgMgr,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Policy {
    pub name: String,
    pub depends_on: String,
    pub map_to: MapCmd,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgMgr {
    pub value: String,
    pub cmds: Cmd,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cmd {
    pub add: String,
    pub list: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapCmd {
    pub value: String,
    pub cmd: Option<String>,
}

pub fn get_project_config(dir: &Path) -> ProjectConfig {
    let bolt_config_filename = "boltconfig.json";

    let err_msg = format!(
        "Could not find a boltconfig.json for: {:?}",
        Path::new(dir).to_str().unwrap()
    );

    let contents = get_config(
        &dir.to_str().unwrap().to_owned(),
        bolt_config_filename,
        err_msg.as_str(),
    );
    let config: ProjectConfig = serde_json::from_str(contents.as_str()).unwrap();

    config
}

pub fn get_workspace_config() -> WorkspaceConfig {
    let root_config = "bolt.json";

    let err_msg = format!(
        "Could not find a root bolt.json in current dir. Is your project initialized with bolt?",
    );

    let contents = get_config(&".".to_owned(), root_config, err_msg.as_str());

    let config: WorkspaceConfig = serde_json::from_str(contents.as_str()).unwrap();

    config
}

fn get_config(dir: &String, file_name: &str, err_msg: &str) -> String {
    let path = Path::new(dir);
    let children = fs_utils::read_file_tree(path).unwrap();

    let target: &Vec<_> = &children
        .iter()
        .filter(|c| c.to_ascii_lowercase().to_str() == Some(file_name))
        .collect();

    if target.is_empty() {
        println!("{}", err_msg.red());

        std::process::exit(1)
    } else {
        let config_path = path.join(file_name);
        let contents = read_to_string(config_path).unwrap();

        contents
    }
}
