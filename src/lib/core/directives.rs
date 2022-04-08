use std::fs::{self, read_to_string};
use std::path::Path;

use cmder::{Designation, Formatter};
use colored::Colorize;

pub fn load_directives(dir: &Path, verbose: bool) {
    let file = dir.join("directives.bolt");

    if !file.exists() {
        if verbose {
            let msg = format!("No directives found for: '{}'\n", dir.to_str().unwrap());
            let mut fmtr = Formatter::default();
            fmtr.add(Designation::Headline, &msg);
            fmtr.print();
        }
        return;
    }

    println!(
        "🔃 {} for: '{}'",
        "Applying directives".green(),
        dir.to_str().unwrap().green()
    );
    let contents = read_to_string(file).unwrap();
    let mut files_to_link: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.starts_with("@apply") {
            let vals: Vec<&str> = line.split(' ').collect();

            for v in &vals[1..] {
                if v.is_empty() {
                    return;
                }
                files_to_link.push(v)
            }
        }
    }

    for file in files_to_link {
        make_link(file, dir)
    }
}

fn make_link(path: &str, target: &Path) {
    let dirs: Vec<&str> = path.split('/').collect();
    let filename = if dirs.is_empty() {
        path
    } else {
        dirs.last().unwrap()
    };
    let target_path = Path::new(target).join(filename);
    let og_path = Path::new("./config").join(path);

    let val = og_path.to_str().unwrap();

    match fs::copy(&og_path, target_path) {
        Ok(_) => {
            let msg = format!("🔗 Linking: {} - Success", val.replace('\\', "/"));
            println!("    {}", msg.cyan());
        }
        Err(_) => {
            let msg = format!("🔗 Linking: {} - Failed", val.replace('\\', "/"));
            println!("    {}", msg.red());
        }
    }
}
