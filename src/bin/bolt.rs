use bolt::cmd;
use bolt::core;

use cmder::{Event, Program};

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .description("A CLI for managing multi-lingual monorepos")
        .author("Victor Ndaba <vndabam@gmail.com>");

    program
        .command("up <app-name>")
        .alias("u")
        .description("A command to start a given app in the workspace")
        .option("-s --skip", "Skip checking/installing the dependencies")
        .option(
            "-a --all",
            "Run tests for all the projects in the workspace",
        )
        .option(
            "-p --priority <priority-value>",
            "The priority to use when starting apps",
        )
        .action(cmd::up)
        .build(&mut program);

    program
        .command("load [app-name]")
        .alias("l")
        .description("A command to load directives for a given app in the workspace")
        .option("-a --all", "Load directives for all the apps")
        .action(cmd::load)
        .build(&mut program);

    program
        .command("ci <app-name>")
        .alias("c")
        .description("Run A CI workflow for the provided app")
        .option("-a --all", "Run CI workflows for all apps in the workspace")
        .action(|_v, _o| {})
        .build(&mut program);

    program
        .command("test <app-name>")
        .alias("t")
        .description("A command to run the configured tests for the provided app")
        .option(
            "-a --all",
            "Run tests for all the projects in the workspace",
        )
        .option(
            "-p --priority <priority-value>",
            "Use the provided priority level to run tests.",
        )
        .action(|vals, opts| {
            dbg!(vals, opts);
        })
        .build(&mut program);

    program.on(Event::UnknownCommand, |_p, _v| {
        let args: Vec<_> = std::env::args().collect();
        let args = args[1..].to_vec();

        core::redirect(&args);
    });

    program.on(Event::OutputHelp, |_p, _v| {
        println!();
    });

    program.parse();
}
