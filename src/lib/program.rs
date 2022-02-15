use super::parser::Command;

use colored::Colorize;

#[derive(Debug)]
/// The crux of the whole cli, contains a commands field that stores all the program commands in a vector
pub struct Program {
    /// Holds all the possible commands of the program
    pub cmds: Vec<Command>,
    /// Contains the version information of the program
    pub version: String,
    /// Contains the author's name
    pub author: String,
    /// A simple string containing the tagline of the program
    pub about: String,
}

impl Program {
    /// Creates a new instance of the program with an empty vector for the cmds field
    pub fn new() -> Self {
        Self {
            cmds: vec![],
            version: "0.1.0".to_owned(),
            author: "Victor Ndaba".to_owned(),
            about: "A CLI for managing projects using Bolt".to_owned(),
        }
    }
    /// This method is used to register a new command to the program
    pub fn add_cmd() -> Command {
        Command::new()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    /// This function is called when the program starts, it creates all the commands of the program.
    pub fn init(&mut self) {
        let hf = "-h | --help |  | Displays the help command";

        Program::add_cmd()
            .command("<app-name>, <command>")
            .alias("<app-alias>")
            .describe("Redirects the passed command to the target app")
            .option(
                "-f | --first |  | If similar apps are found, runs the command on the first app.",
            )
            .option(hf)
            .build(self);

        Program::add_cmd()
            .command("up, <app-name>? ")
            .alias("u")
            .describe("A command for starting projects in the workspace.")
            .option(
                "-p | --priority | <value> | Specifies the priority to use when starting the apps.",
            )
            .option(hf)
            .build(self);

        Program::add_cmd()
            .command("init, <workspace-name> ")
            .alias("i")
            .describe("Creates and bootstraps a new bolt project workspace.")
            .option("-q | --quiet |  | Skips the prompts and sets up the default workspace")
            .option(hf)
            .build(self);

        Program::add_cmd()
            .command("test, <app-name>? ")
            .alias("t")
            .describe("A command for running your configured tests for projects in the workspace.")
            .option("-p | --priority | <value> | Specifies the priority to use to run the tests")
            .option(hf)
            .build(self);
    }

    pub fn parse(&mut self, args: &Vec<String>) -> Option<Vec<&Command>> {
        if Program::validate_args(args, &self.cmds) {
            let internals: Vec<_> = self
                .cmds
                .iter()
                .filter(|cmd| {
                    let cfg: Vec<_> = cmd.name.split(",").collect();
                    let matcher = args[0].to_lowercase();
                    cfg[0] == matcher || cmd.alias == matcher
                })
                .collect();

            if internals.is_empty() {
                return None;
            }

            return Some(internals);
        }

        None
    }

    pub fn validate_args(arguments: &Vec<String>, cmds: &Vec<Command>) -> bool {
        match &arguments.first() {
            Some(val) => {
                let value = val.to_lowercase();
                if Program::is_special_flag(value.as_str(), cmds) {
                    return false;
                }
                true
            }
            None => {
                // Program::output_help(cmds, "You have not passed any command!");
                false
            }
        }
    }

    fn is_special_flag(value: &str, cmds: &Vec<Command>) -> bool {
        if value == "--help" || value == "-h" {
            Program::output_help(&cmds, "");
            return true;
        } else if value == "--version" || value == "-v" {
            Program::output_version();
            return true;
        }
        false
    }

    /// Outputs help for the program, and prints the error if any is passed
    pub fn output_help(cmds: &Vec<Command>, err: &str) {
        println!();
        println!("{}", Program::new().about.yellow());
        println!();
        println!("USAGE: ");
        println!("   bolt [command] [options]");
        println!();
        println!("COMMANDS: ");
        for cmd in cmds {
            let cfg: Vec<_> = cmd.name.split(",").collect();
            println!(
                "  ({} | {}) {} {} ",
                &cfg[0].cyan(),
                cmd.alias.cyan(),
                "[options]".cyan(),
                &cfg[1].cyan()
            );
            println!("     {}", cmd.description);
            println!();
        }
        println!(
            "  Run: bolt {} for detailed usage of a command",
            "<command> --help | -h".cyan()
        );
        println!();
        if !err.is_empty() {
            println!("{}", err.red());
            println!();
        }
    }

    /// Simply prints the version information for the program
    pub fn output_version() {
        println!();
        println!(
            "You are using bolt version: {}",
            Program::new().version.cyan()
        );
        println!();
    }

    /// Outputs help information for a speficic command and prints an optional error
    pub fn output_command_help(cmd: &Command, err: &str) {
        let cfg: Vec<_> = cmd.name.split(",").collect();

        println!();
        println!("{}", cmd.description.yellow());
        println!();
        println!("USAGE: bolt {} [options] {} ", cfg[0], cfg[1]);
        println!();
        println!("OPTIONS: ");
        for opt in &cmd.options {
            if opt.full.is_empty() {
                continue;
            }
            let value = format!("  {}, {} {} ", opt.short, opt.full, opt.params);
            println!("{}", value.cyan());
            println!("  {} ", opt.docstring);
            println!();
        }
        if !err.is_empty() {
            println!("{}", err.red());
            println!();
        }
    }
}
