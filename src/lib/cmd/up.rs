use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::parser;
use super::super::program::Program;
use super::super::utils::{get_config, read_file_tree};
use super::super::utils::{Config, Policy};

use colored::Colorize;

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
            dbg!(&msg);
            execute(&cmd, &env, &full_path, true, &msg);
        }
    }
}

fn execute(cmd: &String, target: &str, dir: &PathBuf, wait: bool, msg: &str) {
    let shell = if target == "windows" { "cmd" } else { "sh" };
    let first_arg = if target == "windows" { "/C" } else { "-c" };

    println!("{}", msg.green());

    let mut command = Command::new(shell)
        .arg(first_arg)
        .arg(cmd)
        .current_dir(dir)
        .spawn()
        .expect("Failed to start the application");

    if wait {
        command.wait().expect("Failed to wait on the child process");
    }
}

fn resolve_policy(cfg: Config, val: String) -> (Option<String>, String, String) {
    let cmd: Vec<&Policy> = cfg.policies.iter().filter(|p| p.name == val).collect();
    let policy = &cmd.first().unwrap();

    let pre_load = if policy.depends_on.is_empty() {
        None
    } else if policy.depends_on == "^" || policy.depends_on == "install" {
        Some(String::from("install"))
    } else {
        Some(policy.depends_on.clone())
    };

    let target = &policy.map_to.value;

    match target.as_str() {
        "cli" => {
            let target_cli = cfg.info.env.cli.unwrap().clone();
            let target_cmd = policy.map_to.cmd.clone().unwrap();

            (
                pre_load,
                format!("{} {}", target_cli, target_cmd),
                "🚀 Starting your application...".to_owned(),
            )
        }
        "pkg_mgr" => {
            let pkgr = cfg.info.env.pkg_mgr;
            let value = pkgr.value;
            let cmd = pkgr.cmds.add;

            (
                pre_load,
                format!("{} {}", value, cmd),
                "➕ Installing dependencies...".to_owned(),
            )
        }
        _ => (None, "".to_owned(), "".to_owned()),
    }
}
