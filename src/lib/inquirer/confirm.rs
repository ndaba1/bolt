use std::io;

use colored::Colorize;

pub fn confirm(msg: &str) -> bool {
    println!("{} {} (y/N)", "?".green(), msg);

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    match buffer.as_str() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => {
            println!("Please enter either y or n");
            std::process::exit(0)
        }
    }
}
