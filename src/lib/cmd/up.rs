use crate::parser::Command;
use super::super::program::Program;


pub fn up(args: &Vec<String>, cmd: &Command) {

    if args.is_empty() {
        Program::output_command_help(cmd, "Missing required argument");
        return;
    }

    let val = args[0].to_lowercase();
    let name = val.as_str();
    match name {
        "--help" | "-h" => Program::output_command_help(cmd, ""),
        _ => println!("Currently running fn up with args: {:?}", args)
    };
}