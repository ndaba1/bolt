use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;

pub fn execute(cmd: String, dir: PathBuf, wait: bool, msg: &str) {
    let target: &str;
    if cfg!(windows) {
        target = "windows"
    } else {
        target = "unix"
    }

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
// /**
//  * To do: implement graceful exit for the child_process when parent process:
//  * SIGNIT, SIGTERM, SIGKILL
//  */
