use std::path::Path;

use colored::Colorize;

use super::super::core;
use super::super::parser;
use super::super::program::Program;
use super::super::utils::ProjectConfig;

pub fn up(cmd: &parser::Command, args: &Vec<String>) {
    if args.is_empty() {
        // dont throw err, instead prompt for opts: run all, enter prio, choose
        Program::output_command_help(cmd, "Missing required arguments");
        return;
    }

    let (target, vals) = cmd.parse(args);

    if vals.contains_key("-h") | vals.contains_key("--help") {
        Program::output_command_help(cmd, "");
        return;
    }

    if target.is_empty() && !vals.contains_key("-p") | !vals.contains_key("--priority") {
        println!(
            "{}",
            "Please pass the name of the project to start, or specify a priority level".red()
        );
        return;
    }
    let (proj_path, config) = core::setup_cmd(&target);
    let mut skip_deps = false;

    if vals.contains_key("-s") | vals.contains_key("--skip") {
        skip_deps = true
    }

    core::load_directives(Path::new(&proj_path.as_str()), false);
    start(config, "up".to_owned(), &proj_path, skip_deps)
}

fn start(cfg: ProjectConfig, value: String, dir: &String, skip: bool) {
    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match core::resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            if !skip {
                start(cfg.clone(), val, &dir, false);
            }
            core::execute(cmd, full_path, true, &msg);
        }
        (None, cmd, msg) => {
            core::execute(cmd, full_path, true, &msg);
        }
    }
}
