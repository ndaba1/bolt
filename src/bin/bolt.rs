#![allow(dead_code)]
#![allow(unused_imports)]

extern crate core;

use std::env;
use bolt::cmd::up;
use bolt::program::Program;
use bolt::parser::{Collection, Command};

fn main() {
    let mut collection = Collection{cmds: vec![]};
    let mut program = Program::new();

    program.init(&mut collection);

    let mut args: Vec<String> = env::args().collect();
    let size = args.len();

    if validate_args(&mut args[1..size].to_vec(), &mut collection) {
        parse_args(&mut args[1..size].to_vec(), &mut collection);
    }


}

fn parse_args(arguments: &mut Vec<String>, col: &mut Collection) {
    let len = arguments.len();
    // Check if passed command is internal
    let mut internals: Vec<_> = col.cmds.iter()
        .filter(|cmd| {
            let name: Vec<_> = cmd.name.split(",").collect();
            let matcher = &arguments[0].to_lowercase();
            &name[0] == matcher || &cmd.alias == matcher
        }).collect();

    if internals.is_empty() {
        eprintln!("Not an internal command");
        return;
    }

    let command = internals.pop().unwrap();
    let cfg: Vec<_> = command.name.split(",").collect();

    resolve_cmd(cfg[0], &command, &arguments[1..len].to_vec())

}

fn validate_args(arguments: &mut Vec<String>, col: &mut Collection) -> bool {
    match &arguments.first() {
        Some(val) => {
            let value = val.to_lowercase();
            if check_special_flags(value.as_str(), col) {
                return false
            }
            true
        },
        None => {
            Program::output_help(col, "You have not passed any command!");
            false
        }
    }
}

fn check_special_flags(value: &str, colc: &mut Collection) -> bool {
    if value == "--help" || value == "-h" {
        Program::output_help(colc, "");
        return true
    } else if value == "--version" || value == "-v" {
        Program::output_version();
        return true
    }
    false
}

fn resolve_cmd(name: &str, cmd: &Command, args: &Vec<String>) {
    match name {
        "up" => up(args, cmd),
        _ => println!("Something else")
    }
}
