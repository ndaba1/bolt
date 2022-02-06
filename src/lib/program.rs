use super::parser::{commands, Command, Flag};
use crate::parser::commands::Collection;
use super::cmd::up;

const BOLT_VERSION: &str = "0.1.0";
const BOLT_TAG_LINE: &str = "A CLI for bootstrapping projects using Bolt";

#[derive(Debug)]
pub struct Program {
    pub cmds: Vec<Command>,
}

impl Program {
    pub fn new() -> Self {
        Self { cmds: vec![] }
    }
}

impl Program {
    pub fn init(&mut self, col: &mut Collection) {

        let help_flag = Flag {
            short: "-h".to_owned(),
            full: "--help".to_owned(),
            params: "".to_owned(),
            docstring: "Displays help for the command its passed to".to_owned()
        };


        let mut cmd = Command::new();
        cmd
           .command("up, <app-name>? ".to_owned())
             .alias("u".to_owned())
           .describe("Starts running the selected app or the configured apps".to_owned())
           .option(
               Flag{
                   short: "-p".to_owned(),
                   full: "--priority".to_owned(),
                   params: "<value>".to_owned(),
                   docstring: "Specifies the priority on which to run the apps".to_owned()
               }
           )
            .option(help_flag.clone()).action(|args|{}, vec![]);
        col.cmds.push(cmd);

        let mut cmd = Command::new();
        cmd
            .command("init, <workspace-name> ".to_owned())
            .alias("i".to_owned())
            .describe("Creates and bootstraps a new bolt project workspace.".to_owned())
            .option(
                Flag{
                    short: "-q".to_owned(),
                    full: "--quiet".to_owned(),
                    params: "".to_owned(),
                    docstring: "Skip the prompts and setup the default workspace".to_owned()
                }
            )
            .option(help_flag.clone());
        col.cmds.push(cmd);

        let mut cmd = Command::new();
        cmd
            .command("test, <app-name>? ".to_owned())
            .alias("t".to_owned())
            .describe("Runs the configured tests on the given app or the selected apps".to_owned())
            .option(
                Flag{
                    short: "-p".to_owned(),
                    full: "--priority".to_owned(),
                    params: "<value>".to_owned(),
                    docstring: "Specifies the priority on which to run the apps".to_owned()
                }
            )
            .option(help_flag.clone());
        col.cmds.push(cmd)
    }

    pub fn output_help(col: &Collection, err: &str) {
        println!();
        println!("{}", BOLT_TAG_LINE);
        println!();
        println!("USAGE: ");
        println!("   bolt [command] [options]");
        println!();
        println!("COMMANDS: ");
        println!();
        for cmd in &col.cmds {
            let cfg: Vec<_> = cmd.name.split(",").collect();
            println!("({} | {}) [options] {} ",
                     &cfg[0], cmd.alias,
                     &cfg[1]);
            println!("  {}", cmd.description);
            println!();
        }
        println!("Run: bolt <command> --help | -h for specific command help.");
        println!();
        if !err.is_empty() {
            println!("  {}", err);
            println!();
        }
    }

    pub fn output_version() {
        println!();
        println!("You are using bolt version: {}", BOLT_VERSION);
        println!();
    }

    pub fn output_command_help(cmd: &Command, err: &str) {
        let cfg: Vec<_> = cmd.name.split(",").collect();

        println!();
        println!("USAGE: bolt {} [options] {} ", cfg[0], cfg[1]);
        println!();
        println!("{}", cmd.description);
        println!();
        println!("OPTIONS: ");
        for opt in &cmd.options {
            if opt.full.is_empty() {
                continue;
            }
            println!("  {}, {} {} ", opt.short, opt.full, opt.params);
            println!("  {} ", opt.docstring);
            println!();
        }
        if !err.is_empty() {
            println!("  {}", err);
            println!();
        }

    }

}

