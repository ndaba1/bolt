use std::path::Path;

use super::super::core;
use super::super::parser;
use super::super::utils::ProjectConfig;

pub fn test(cmd: &parser::Command, args: &Vec<String>) {
    let (target, _vals) = cmd.parse(args);
    let (proj_path, config) = core::setup_cmd(&target);

    core::load_directives(Path::new(&proj_path.as_str()), false);
    start(config, "test".to_owned(), &proj_path)
}

fn start(cfg: ProjectConfig, value: String, dir: &String) {
    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match core::resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            start(cfg.clone(), val, &dir);
            core::execute(cmd, full_path, true, &msg);
        }
        (None, cmd, msg) => {
            core::execute(cmd, full_path, true, &msg);
        }
    }
}
