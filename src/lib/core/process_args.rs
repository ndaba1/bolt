use crate::parser::Command;

pub fn process_args(cmd: &Command, args: &Vec<String>) -> (Vec<String>, String) {
    let flags: Vec<String> = args
        .iter()
        .map(|x| x.to_owned())
        .filter(|x| {
            let val = x.clone().to_owned();
            let full: Vec<String> = cmd.options.iter().map(|f| f.full.clone()).collect();
            let short: Vec<String> = cmd.options.iter().map(|f| f.short.clone()).collect();

            full.contains(&val) || short.contains(&val)
        })
        .collect();

    let mut name = String::from("");
    for arg in args {
        if !flags.contains(&arg) {
            name = arg.clone()
        }
    }

    (flags, name)
}
