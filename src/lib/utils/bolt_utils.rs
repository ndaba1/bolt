use super::fs_utils::read_file_tree;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
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

pub fn get_config(dir: &Path) -> Config {
    let bolt_config_filename = "boltconfig.json";
    let children = read_file_tree(dir).unwrap();

    let target: &Vec<_> = &children
        .iter()
        .filter(|c| c.to_ascii_lowercase().to_str() == Some(bolt_config_filename))
        .collect();

    if target.is_empty() {
        let err_msg = format!(
            "Could not find a boltconfig.json in dir: {:?}",
            Path::new(dir).to_str().unwrap()
        );
        println!("{}", err_msg.red());
        std::process::exit(1)
    }

    let config_path = dir.join(bolt_config_filename);

    let contents = read_to_string(config_path).unwrap();

    let config: Config = serde_json::from_str(contents.as_str()).unwrap();

    config
}
