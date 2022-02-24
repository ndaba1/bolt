use std::path::Path;

use super::super::core;
use super::super::parser;
use super::super::program::Program;

pub fn load(cmd: &parser::Command, args: &Vec<String>) {
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
            let (proj_path, _config) = core::setup_cmd(name);
            core::load_directives(Path::new(proj_path.as_str()), true);
        }
    };
}
