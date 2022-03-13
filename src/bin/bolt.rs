use bolt::cmd::{load, up};

use cmder::{Event, Program};

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .description("A CLI for managing multi-lingual monorepos")
        .author("Victor Ndaba <vndabam@gmail.com>");

    program
        .add_cmd()
        .command("up <app-name>")
        .alias("u")
        .describe("A command to start a given app in the workspace")
        .option("-s --skip", "Skip checking/installing the dependencies")
        .option(
            "-a --all",
            "Run tests for all the projects in the workspace",
        )
        .option(
            "-p --priority <priority-value>",
            "The priority to use when starting apps",
        )
        .action(|vals, opts| up(vals, opts))
        .build(&mut program);

    program
        .add_cmd()
        .command("load [app-name]")
        .alias("l")
        .describe("A command to load directives for a given app in the workspace")
        .option("-a --all", "Load directives for all the apps")
        .action(|vals, opts| load(vals, opts))
        .build(&mut program);

    program
        .add_cmd()
        .command("test <app-name>")
        .alias("t")
        .describe("A command to run the configured tests for the provided app")
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

    program.on(Event::OutputHelp, |_p, _v| {
        println!();
    });

    program.parse();
}
