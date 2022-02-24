use std::path::Path;

use colored::Colorize;

use crate::core::process_args::process_args;

use super::super::core;
use super::super::parser;
use super::super::program::Program;
use super::super::utils::ProjectConfig;

struct TestConfig {
    // help: bool,
    skip: bool,
}

pub fn test(args: &Vec<String>, cmd: &parser::Command) {
    if args.is_empty() {
        // dont throw err, instead prompt for opts: run all, enter prio, choose
        Program::output_command_help(cmd, "Missing required argument");
        return;
    }

    let (flags, name) = process_args(cmd, args);
    let mut opts = TestConfig {
        // help: false,
        skip: false,
    };

    for flag in flags {
        match flag.as_str() {
            "-s" | "--skip" => opts.skip = true,
            "-h" | "--help" => {
                Program::output_command_help(cmd, "");
                return;
            }
            _ => continue,
        }
    }

    if name.is_empty() {
        println!("{}", "Please pass the name of the project to test".red());
        return;
    }

    let (proj_path, config) = core::setup_cmd(&name);
    core::load_directives(Path::new(&proj_path.as_str()), false);
    start(config, "test".to_owned(), &proj_path)
}

fn start(cfg: ProjectConfig, value: String, dir: &String) {
    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match core::resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            start(cfg.clone(), val, &dir);
            core::execute(&cmd, &full_path, true, &msg);
        }
        (None, cmd, msg) => {
            core::execute(&cmd, &full_path, true, &msg);
        }
    }
}
