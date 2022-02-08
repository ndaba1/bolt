use super::super::program::Program;
use super::super::utils::{get_config, read_file_tree};
use super::super::utils::{Config, Policy};
use crate::parser;
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn up(args: &Vec<String>, cmd: &parser::Command) {
    if args.is_empty() {
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
            start(config, &path)
        }
    };
}

// Read boltconfing, get cli, get policy, execute it
fn start(cfg: Config, dir: &String) {
    let cmd: Vec<&Policy> = cfg
        .policies
        .iter()
        .filter(|p| p.name == "up".to_owned())
        .collect();

    let root_path = std::env::current_dir().unwrap();
    let target_cli = cfg.info.env.cli.unwrap();
    let cli_cmd = &cmd.first().unwrap().map_to;

    if target_cli.is_empty() {
        println!("No cli present, exiting...");
    }
    let full_path = Path::new(&root_path).join(&dir);
    let cmd_args = vec![target_cli, cli_cmd.clone()];

    if cfg!(target_os = "windows") {
        execute(cmd_args, "windows", full_path)
    } else {
        execute(cmd_args, "not-windows", full_path)
    };
}

fn execute(args: Vec<String>, target: &str, dir: PathBuf) {
    let shell = if target == "windows" { "cmd" } else { "sh" };
    let first_arg = if target == "windows" { "/C" } else { "-c" };

    println!("{}", "🚀 Starting your application...".green());
    let mut command = Command::new(shell)
        .arg(first_arg)
        .args(args)
        .current_dir(dir)
        .spawn()
        .expect("Failed to start the application");

    command.wait().expect("Failed to wait on the child process");
    println!("Bolt exited successfully!");
}
