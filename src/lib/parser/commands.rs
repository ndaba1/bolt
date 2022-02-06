use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub alias: String,
    pub description: String,
    pub options: Vec<Flag>,
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub short: String,
    pub full: String,
    pub params: String,
    pub docstring: String
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub cmds: Vec<Command>
}

impl Command {
    pub fn command(&mut self, name: String) -> &mut Command {
        self.name = name;
        self
    }
    pub fn alias(&mut self, val: String) -> &mut Command {
        self.alias = val;
        self
    }
    pub fn describe(&mut self, desc: String) -> &mut Command {
        self.description = desc;
        self
    }
    pub fn option(&mut self, opt: Flag) -> &mut Command {
        let flag = Flag {
            short: opt.short,
            full: opt.full,
            params: opt.params,
            docstring: opt.docstring
        };
        self.options.push(flag);
        self
    }
    pub fn action<F>(&self, func: F, args: Vec<String>)
        where F: Fn(Vec<String>) {
        func(args)
    }

}

impl Command {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            alias: "".to_owned(),
            description: "".to_owned(),
            options: vec![Flag{short: "".to_owned(), full: "".to_owned(), params: "".to_owned(), docstring: "".to_owned()}],
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! build_fn {
($func_name:ident, $args:item) => {
    fn $func_name($args){}
};
}