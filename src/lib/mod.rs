/// The cmd module contains the implementations of the commands, each in its own module
pub mod cmd;

/// A set of general utilities for the program, such as reading and writing to the fs
pub mod utils;

/// The core module contains all the important executions of the program such as spawning of a new thread to execute a command
pub mod core;

mod hooks;