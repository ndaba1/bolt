use std::path::Path;

use super::super::core;
use super::super::parser;
use super::super::program::Program;
use super::super::utils::ProjectConfig;

pub fn up(args: &Vec<String>, cmd: &parser::Command) {
    if args.is_empty() {
        // dont throw err, instead prompt for opts: run all, enter prio, choose
        Program::output_command_help(cmd, "Missing required argument");
        return;
    }

    let val = args[0].to_lowercase();
    let name = val.as_str();

    match name {
        "--help" | "-h" => Program::output_command_help(cmd, ""),
        _ => {
            let (proj_path, config) = core::setup_cmd(name);
            start(config, "up".to_owned(), &proj_path)
        }
    };
}

fn start(cfg: ProjectConfig, value: String, dir: &String) {
    let env;

    if cfg!(target_os = "windows") {
        env = "windows"
    } else {
        env = "unix"
    };

    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match core::resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            start(cfg.clone(), val, &dir);
            core::execute(&cmd, &env, &full_path, true, &msg);
        }
        (None, cmd, msg) => {
            core::execute(&cmd, &env, &full_path, true, &msg);
        }
    }
}
