use super::super::program::Program;
use super::super::utils::{get_config, read_file_tree};
use crate::parser::Command;
use std::path::Path;

pub fn up(args: &Vec<String>, cmd: &Command) {
    if args.is_empty() {
        Program::output_command_help(cmd, "Missing required argument");
        return;
    }

    let val = args[0].to_lowercase();
    let name = val.as_str();

    match name {
        "--help" | "-h" => Program::output_command_help(cmd, ""),
        _ => {
            let path = format!("./projects/{}", &name);
            let results = read_file_tree(Path::new("./projects")).unwrap();

            let target: Vec<_> = results
                .iter()
                .filter(|r| match r.to_ascii_lowercase().to_str() {
                    Some(val) => val == name,
                    None => false,
                })
                .collect();

            if target.is_empty() {
                eprintln!("Could not find target directory: {}", &name);
                return;
            }

            get_config(Path::new(&path)).unwrap();
        }
    };
}
