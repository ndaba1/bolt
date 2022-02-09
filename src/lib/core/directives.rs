use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

use colored::Colorize;

pub fn load_directives(dir: &PathBuf) {
    println!("🔃 {}", "Applying directives...".green());
    let file = dir.join("directives.bolt");

    let contents = read_to_string(file).unwrap();
    let mut files_to_link: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.starts_with("@apply") {
            let vals: Vec<&str> = line.split(" ").collect();
            let len = vals.len();

            for v in &vals[1..len] {
                if v.is_empty() {
                    return;
                }
                files_to_link.push(&v)
            }
        }
    }

    for file in files_to_link {
        make_link(file, dir)
    }
}

fn make_link(path: &str, target: &PathBuf) {
    let dirs: Vec<&str> = path.split("/").collect();
    let filename = if dirs.is_empty() {
        path
    } else {
        dirs.last().unwrap()
    };
    let target_path = Path::new(target).join(filename);
    let og_path = Path::new("./config").join(path);

    let msg = format!("🔗 Linking: {:?}", &og_path);
    println!("  {}", msg.green());

    match fs::copy(og_path, target_path) {
        Ok(_) => return,
        Err(_) => println!("Failed to link file"),
    }
}
