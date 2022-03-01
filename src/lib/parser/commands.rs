use std::collections::HashMap;
use std::fmt::Debug;

use super::super::program::Program;

#[derive(Clone)]
/// The structure of a command in the program
pub struct Command {
    /// The name of the command i.e up, start, run
    pub name: String,
    /// Any alias given to the command, normally a single letter, u, i, s
    pub alias: String,
    /// A simple description of the command, gets output when the --help flag is passed
    pub description: String,
    /// Options are the flags that can be passed to the specified command i.e -q --priority
    pub options: Vec<Flag>,
    /// Stores the actual function that is to be invoked
    pub callback: fn(&Command, &Vec<String>) -> (),
}

#[derive(Debug, Clone)]
/// Specifies the structure of a flag passed to a command
pub struct Flag {
    /// The short version of the flag, normally with a single hyphen
    pub short: String,
    /// The full version of the flag, normally specified with two leading hyphens
    pub full: String,
    /// The parameters that the option accepts if any
    pub params: String,
    /// A description of the option and the parameters it accepts
    pub docstring: String,
}

/// Methods for command mutation
impl Command {
    /// Changes the name of a command to the passed value
    pub fn command(&mut self, name: &str) -> &mut Command {
        self.name = name.to_owned();
        self
    }

    /// Mutates the alias of the command to which it is attached
    pub fn alias(&mut self, val: &str) -> &mut Command {
        self.alias = val.to_owned();
        self
    }

    /// Edits the description of a command, setting it to the passed value
    pub fn describe(&mut self, desc: &str) -> &mut Command {
        self.description = desc.to_owned();
        self
    }

    /// Pushes new options to the options vector of a command
    pub fn option(&mut self, body: &str) -> &mut Command {
        let opt: Vec<_> = body.split("|").collect();

        let flag = Flag {
            short: opt[0].trim().to_owned(),
            full: opt[1].trim().to_owned(),
            params: opt[2].trim().to_owned(),
            docstring: opt[3].trim().to_owned(),
        };

        let pre = self.options.last().unwrap();
        if pre.short.is_empty() {
            self.options = vec![flag]
        } else {
            self.options.push(flag);
        }

        self
    }

    /// Takes a mutable reference to the program struct and pushes the constructed command to the the `cmds` field in the struct
    pub fn build(&mut self, prog: &mut Program) {
        let val = self.clone();
        prog.cmds.push(val)
    }

    /// Takes a closure that invokes the callback function to execute functionality
    pub fn action(&mut self, cb: fn(&Command, &Vec<String>) -> ()) -> &mut Command {
        let action = cb;
        self.callback = action;
        self
    }

    /// Takes the args passed to the program as input, checks for flags and returns a hashmap containing the config for each of the flags, and the target name if any
    pub fn parse(&self, raw_args: &Vec<String>) -> (String, HashMap<String, Option<String>>) {
        if raw_args.is_empty() {
            Program::output_command_help(self, "Missing required arguments");
            std::process::exit(1);
        }

        let mut switches: Vec<String> = vec![];
        let mut config: HashMap<String, Option<String>> = HashMap::new();

        for f in &self.options {
            for arg in raw_args.iter().enumerate() {
                if arg.1 == &f.short || arg.1 == &f.full {
                    config.insert(arg.1.clone(), None);

                    if !f.params.is_empty() {
                        config.insert(arg.1.clone(), Some(raw_args[arg.0 + 1].clone()));
                    }
                    switches.push(arg.1.clone())
                }
            }
        }

        let mut name = String::from("");
        for arg in raw_args {
            if !switches.contains(&arg) {
                name = arg.clone()
            }
        }

        (name, config)
    }
}

impl Command {
    /// Returns a new instance of an empty command for its values to be modified
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            alias: "".to_owned(),
            description: "".to_owned(),
            options: vec![Flag {
                short: "-h".to_owned(),
                full: "--help".to_owned(),
                params: "".to_owned(),
                docstring: "Displays the help command".to_owned(),
            }],
            callback: Self::init,
        }
    }

    pub fn init(&self, _args: &Vec<String>) {}
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}
