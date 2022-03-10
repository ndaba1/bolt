use std::collections::HashMap;
use std::path::Path;

use super::super::core;
use super::super::utils;

pub fn load(vals: HashMap<String, String>, opts: HashMap<String, String>) {
    let target = vals.get("app_name");

    match target {
        Some(value) => {
            let (proj_path, _config) = core::setup_cmd(value);
            core::load_directives(Path::new(proj_path.as_str()), true);
        }
        None => {
            if opts.contains_key("all") {
                for p in get_projects() {
                    let (proj_path, _config) = core::setup_cmd(&p);
                    core::load_directives(Path::new(proj_path.as_str()), true);
                }
                return;
            } else {
                println!("Please provide an app-name or pass the -a flag to load all directives");
            }
        }
    }
}

fn get_projects() -> Vec<String> {
    let wp_cfg = utils::get_workspace_config();

    match wp_cfg.registered {
        Some(v) => v,
        None => {
            let src_name = &wp_cfg.source[0];
            let results = utils::read_file_tree(Path::new(src_name.as_str())).unwrap();
            let vals = results
                .iter()
                .map(|s| s.to_ascii_lowercase().to_str().unwrap().to_owned())
                .collect();
            vals
        }
    }
}
