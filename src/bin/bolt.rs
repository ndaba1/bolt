use std::env;

use bolt::parser::Command;
use bolt::program::Program;

use bolt::core;

/// Creates a new instance of program and initializes it to build all commands.
fn main() {
    let args: Vec<String> = env::args().collect();
    let refined_args = args[1..].to_vec();

    let mut program = Program::new();
    program.init();

    let config = program.parse(&refined_args);
    match config {
        Some(val) => resolve_cmd(&val, &refined_args),
        None => redirect_cmd(&program, &refined_args),
    }
}

/// Matches through all the internal commands and calls the appropriate method.
fn resolve_cmd(cfg: &Vec<&Command>, args: &Vec<String>) {
    let command = cfg.first().unwrap();
    (command.callback)(command, &args[1..].to_vec());
}

fn redirect_cmd(prog: &Program, args: &Vec<String>) {
    if args.is_empty() {
        Program::output_help(&prog.cmds, "You did not pass any command!");
        return;
    }
    core::redirect(args)
}
