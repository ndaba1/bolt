/// This module contains implementations for the program struct and creates the actual program commands
/// When the program starts, a new instance of the program struct is created and used to parse commands
/// It also contains the functions for outputting general command help
pub mod program;

/// The cmd module contains the implementations of the commands, each in its own module
pub mod cmd;

/// Contains a set of utility functions used for creating a command, each function returns a mutable reference to self allowing multiple methods to be chained together
pub mod parser;

/// A set of general utilities for the program, such as reading and writing to the fs
pub mod utils;
