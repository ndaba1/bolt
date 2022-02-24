use std::path::Path;

use super::{execute, setup_cmd};

use colored::Colorize;

pub fn redirect(args: &Vec<String>) {
    let name = &args[0];

    let (proj_path, _config) = setup_cmd(name.as_str());
    let root_path = std::env::current_dir().unwrap();
    let full_path = Path::new(&root_path).join(&proj_path);

    if !(args.len() > 1) {
        println!("{}", "You did not pass any arguments.".red());
        std::process::exit(1)
    }

    let mut command = String::new();
    for arg in &args[1..] {
        command.push_str(arg);
        command.push_str(&" ")
    }

    let msg = format!(
        "~> Running '{}' in '{}'",
        &command.trim().cyan(),
        &proj_path.cyan()
    );

    execute(&command, &full_path, true, &msg.as_str())
}
