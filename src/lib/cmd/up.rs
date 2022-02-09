use std::path::Path;

use super::super::core::execute;
use super::super::core::resolve_policy;
use super::super::parser;
use super::super::program::Program;
use super::super::utils::Config;
use super::super::utils::{get_config, read_file_tree};

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
            let path = format!("projects/{}/", &name);
            let results = read_file_tree(Path::new("projects")).unwrap();

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

            let config = get_config(&Path::new(path.as_str()));
            start(config, "up".to_owned(), &path)
        }
    };
}

// Read boltconfing, get cli, get policy, execute it
fn start(cfg: Config, value: String, dir: &String) {
    let env;

    if cfg!(target_os = "windows") {
        env = "windows"
    } else {
        env = "unix"
    };

    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&dir);

    match resolve_policy(cfg.clone(), value) {
        (Some(val), cmd, msg) => {
            start(cfg.clone(), val, &dir);
            execute(&cmd, &env, &full_path, true, &msg);
        }
        (None, cmd, msg) => {
            execute(&cmd, &env, &full_path, true, &msg);
        }
    }
}
