use std::path::PathBuf;
use std::process::Command;

use cmder::{Designation, Formatter, Theme};

pub fn execute(cmd: String, dir: PathBuf, wait: bool, msg: &str) {
    let target = if cfg!(windows) { "windows" } else { "unix" };

    let shell = if target == "windows" { "cmd" } else { "sh" };
    let first_arg = if target == "windows" { "/C" } else { "-c" };

    let mut fmtr = Formatter::new(Theme::default());
    fmtr.add(Designation::Headline, msg);
    fmtr.print();

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
