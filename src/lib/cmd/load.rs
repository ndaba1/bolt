use std::path::Path;

use super::super::core;
use super::super::parser;
use super::super::program::Program;
use super::super::utils;

use colored::Colorize;

pub fn load(cmd: &parser::Command, args: &Vec<String>) {
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

    if vals.contains_key("-a") | vals.contains_key("--all") {
        for p in get_projects() {
            let (proj_path, _config) = core::setup_cmd(&p);
            core::load_directives(Path::new(proj_path.as_str()), true);
        }
        return;
    }

    if target.is_empty() && !vals.contains_key("-a") | !vals.contains_key("--all") {
        println!(
            "{}",
            "Please specify at least one project or pass the -a flag to load all directives".red()
        );
        return;
    }

    let (proj_path, _config) = core::setup_cmd(&target);
    core::load_directives(Path::new(proj_path.as_str()), true);
}

fn get_projects() -> Vec<String> {
    let wp_cfg = utils::get_workspace_config();

    match wp_cfg.registered {
        Some(v) => v,
        None => {
            let src_name = &wp_cfg.source[0];
            let results = utils::read_file_tree(Path::new(src_name.as_str())).unwrap();
            let vals = results
                .iter()
                .map(|s| s.to_ascii_lowercase().to_str().unwrap().to_owned())
                .collect();
            vals
        }
    }
}
