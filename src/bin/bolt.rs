use std::env;

use bolt::parser::Command;
use bolt::program::Program;

use bolt::cmd;
use bolt::core;

/// Creates a new instance of program and initializes it to build all commands.
fn main() {
    let mut program = Program::new();
    program.init();

    let args: Vec<String> = env::args().collect();
    let size = args.len();

    let refined_args = args[1..size].to_vec();

    let config = program.parse(&refined_args);
    match config {
        Some(val) => resolve_cmd(&val, &refined_args),
        None => redirect_cmd(&program, &refined_args),
    }
}

/// Matches through all the internal commands and calls the appropriate method.
fn resolve_cmd(cfg: &Vec<&Command>, args: &Vec<String>) {
    let command = cfg.first().unwrap();

    let config: Vec<_> = command.name.split(",").collect();
    let arr_len = args.len();

    let refined_args = &args[1..arr_len].to_vec();
    // (command.callback)(command, refined_args);

    match config[0] {
        "up" => cmd::up(command, refined_args),
        "load" => cmd::load(command, refined_args),
        _ => println!("Something else"),
    }
}

fn redirect_cmd(prog: &Program, args: &Vec<String>) {
    if args.is_empty() {
        Program::output_help(&prog.cmds, "You did not pass any command!");
        return;
    }
    core::redirect(args)
}
